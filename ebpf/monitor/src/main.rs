#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::{
        bpf_get_current_comm, bpf_get_current_pid_tgid, bpf_get_current_uid_gid, bpf_ktime_get_ns,
    },
    macros::{map, tracepoint},
    maps::{Array, HashMap, PerCpuArray, RingBuf},
    programs::TracePointContext,
};
use ebpf_monitor_common::{Config, EXEC, EXIT, Event, Stats};

#[map]
static CONFIG: Array<Config> = Array::with_max_entries(1, 0);
#[map]
static STARTED: HashMap<u32, Event> = HashMap::with_max_entries(16_384, 0);
#[map]
static STATS: PerCpuArray<Stats> = PerCpuArray::with_max_entries(1, 0);
#[map]
static EVENTS: RingBuf = RingBuf::with_byte_size(1024 * 1024, 0);

#[inline(always)]
fn update_stats(executions: u64, completed: u64, elapsed: u64, dropped: u64, errors: u64) {
    if let Some(ptr) = STATS.get_ptr_mut(0) {
        // SAFETY: per-CPU map，当前 tracepoint 执行期间不会迁移 CPU。
        unsafe {
            (*ptr).executions += executions;
            (*ptr).completed += completed;
            (*ptr).total_ns += elapsed;
            (*ptr).dropped_events += dropped;
            (*ptr).tracking_errors += errors;
        }
    }
}

#[inline(always)]
fn emit(event: &Event) {
    if EVENTS.output(event, 0).is_err() {
        update_stats(0, 0, 0, 1, 0);
    }
}

#[tracepoint]
pub fn monitor_exec(_ctx: TracePointContext) -> u32 {
    let pid = (bpf_get_current_pid_tgid() >> 32) as u32;
    // exec 替换进程映像，旧程序不再等待 exit 结算。
    let _ = STARTED.remove(&pid);
    let Some(config) = CONFIG.get(0) else {
        return 0;
    };
    let uid = bpf_get_current_uid_gid() as u32;
    if uid != config.uid {
        return 0;
    }
    let Ok(command) = bpf_get_current_comm() else {
        update_stats(0, 0, 0, 0, 1);
        return 0;
    };
    // 固定长度比较，无动态字符串或 tracepoint 字段偏移依赖。
    if command != config.command {
        return 0;
    }
    let event = Event {
        timestamp_ns: unsafe { bpf_ktime_get_ns() },
        elapsed_ns: 0,
        pid,
        uid,
        command,
        kind: EXEC,
        reserved: 0,
    };
    update_stats(1, 0, 0, 0, 0);
    if STARTED.insert(&pid, &event, 0).is_err() {
        update_stats(0, 0, 0, 0, 1);
    }
    emit(&event);
    0
}

#[tracepoint]
pub fn monitor_exit(_ctx: TracePointContext) -> u32 {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32;
    if pid != pid_tgid as u32 {
        return 0;
    }
    // SAFETY: 相同 TGID 的 exec 与主线程 exit 不会同时运行；立即复制，
    // 不保留 map 引用跨越 remove，用户端不修改 STARTED。
    let Some(mut event) = (unsafe { STARTED.get(&pid).copied() }) else {
        return 0;
    };
    let now = unsafe { bpf_ktime_get_ns() };
    event.elapsed_ns = now - event.timestamp_ns;
    event.timestamp_ns = now;
    event.kind = EXIT;
    if STARTED.remove(&pid).is_err() {
        update_stats(0, 0, 0, 0, 1);
    }
    update_stats(0, 1, event.elapsed_ns, 0, 0);
    emit(&event);
    0
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
