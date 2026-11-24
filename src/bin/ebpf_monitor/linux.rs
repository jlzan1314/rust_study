use super::{Args, config, decode_event};
use aya::{
    Ebpf,
    maps::{Array, MapData, PerCpuArray, RingBuf},
    programs::{TracePoint, trace_point::TracePointLink},
};
use ebpf_monitor_common::{EXEC, EXIT, Event, Stats};
use std::{
    error::Error,
    io::{self, Write},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::signal::unix::{SignalKind, signal};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn attach(bpf: &mut Ebpf, name: &str, event: &str) -> Result<TracePointLink> {
    let program: &mut TracePoint = bpf
        .program_mut(name)
        .ok_or_else(|| io::Error::other(format!("ELF 缺少程序 {name}")))?
        .try_into()?;
    program.load()?;
    let link_id = program.attach("sched", event)?;
    Ok(program.take_link(link_id)?)
}

// bpf_ktime_get_ns 使用 CLOCK_MONOTONIC，将它换算为启动时校准的 Unix 时间。
// 耗时直接使用内核单调时钟，不受用户修改系统时间影响。
fn clock_offset_ns() -> io::Result<i128> {
    let mut mono: libc::timespec = unsafe { std::mem::zeroed() };
    // SAFETY: mono 指向有效且可写的 timespec。
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut mono) } != 0 {
        return Err(io::Error::last_os_error());
    }
    let wall = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?;
    Ok(wall.as_nanos() as i128 - (mono.tv_sec as i128 * 1_000_000_000 + mono.tv_nsec as i128))
}

fn print_event(out: &mut impl Write, event: Event, offset: i128) -> io::Result<()> {
    let time = offset + event.timestamp_ns as i128;
    let command_end = event.command.iter().position(|&b| b == 0).unwrap_or(16);
    let command = String::from_utf8_lossy(&event.command[..command_end]);
    let kind = match event.kind {
        EXEC => "EXEC",
        EXIT => "EXIT",
        other => {
            return Err(io::Error::other(format!(
                "未知事件类型 {other}，请重新编译两端"
            )));
        }
    };
    write!(
        out,
        "{kind} unix_time={}.{:09} uid={} pid={} comm={}",
        time.div_euclid(1_000_000_000),
        time.rem_euclid(1_000_000_000),
        event.uid,
        event.pid,
        command
    )?;
    if event.kind == EXIT {
        write!(
            out,
            " elapsed_ms={:.3}",
            event.elapsed_ns as f64 / 1_000_000.0
        )?;
    }
    writeln!(out)
}

fn drain(
    events: &mut RingBuf<MapData>,
    out: &mut impl Write,
    offset: i128,
    limit: usize,
) -> io::Result<()> {
    // 限制每轮工作量，持续高流量时也能处理退出信号。
    for _ in 0..limit {
        let Some(bytes) = events.next() else { break };
        let event = decode_event(&bytes)
            .ok_or_else(|| io::Error::other("事件布局不匹配，请重新编译用户端和 eBPF 程序"))?;
        print_event(out, event, offset)?;
    }
    out.flush()
}

pub async fn run(args: Args) -> Result<()> {
    // 在挂载探针前注册信号处理，Ctrl-C/SIGTERM 都会正常卸载和汇总。
    let mut interrupt = signal(SignalKind::interrupt())?;
    let mut terminate = signal(SignalKind::terminate())?;
    let offset = clock_offset_ns()?;
    // Linux 5.11 之前 Map 内存受 memlock 限制；尽力提升，失败时由加载错误反馈。
    let limit = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    // SAFETY: limit 指向有效的 rlimit，不修改其他资源限制。
    let _ = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &limit) };
    let mut bpf = Ebpf::load_file(&args.object).map_err(|error| {
        io::Error::other(format!(
            "加载 {} 失败：{error}；请确认已编译 eBPF ELF、Linux >= 5.8 且有 BPF 权限",
            args.object.display()
        ))
    })?;
    {
        let mut map = Array::try_from(bpf.map_mut("CONFIG").ok_or("缺少 CONFIG map")?)?;
        map.set(0, config(&args), 0)?;
    }
    let mut events = RingBuf::try_from(bpf.take_map("EVENTS").ok_or("缺少 EVENTS map")?)?;
    let stats = PerCpuArray::<_, Stats>::try_from(bpf.take_map("STATS").ok_or("缺少 STATS map")?)?;
    // 先监听退出，再监听执行，避免先记录执行却尚未挂载退出探针。
    let exit_link = attach(&mut bpf, "monitor_exit", "sched_process_exit")?;
    let exec_link = attach(&mut bpf, "monitor_exec", "sched_process_exec")?;
    let mut out = io::BufWriter::new(io::stdout());
    writeln!(
        out,
        "Monitoring UID={} comm={}; Ctrl-C to stop",
        args.uid, args.command
    )?;
    out.flush()?;
    let mut interval = tokio::time::interval(Duration::from_millis(50));
    loop {
        tokio::select! {
            _ = interrupt.recv() => break,
            _ = terminate.recv() => break,
            _ = interval.tick() => drain(&mut events, &mut out, offset, 4096)?,
        }
    }
    // 先停止生产事件，然后排空 RingBuf，读取每 CPU 的内核统计。
    drop(exec_link);
    drop(exit_link);
    drain(&mut events, &mut out, offset, usize::MAX)?;
    let mut total = Stats::default();
    for cpu in stats.get(&0, 0)?.iter() {
        total.executions += cpu.executions;
        total.completed += cpu.completed;
        total.total_ns += cpu.total_ns;
        total.dropped_events += cpu.dropped_events;
        total.tracking_errors += cpu.tracking_errors;
    }
    writeln!(
        out,
        "\nexecutions={} completed={} total_ms={:.3} dropped_events={} tracking_errors={}",
        total.executions,
        total.completed,
        total.total_ns as f64 / 1_000_000.0,
        total.dropped_events,
        total.tracking_errors
    )?;
    out.flush()?;
    Ok(())
}
