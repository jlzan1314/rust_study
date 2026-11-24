//! Aya 原生 eBPF 用户命令监控，详见 docs/ebpf_monitor.md。
use clap::Parser;
#[cfg(any(target_os = "linux", test))]
use ebpf_monitor_common::{Config, Event};
use std::{error::Error, path::PathBuf, process::ExitCode};

#[cfg(target_os = "linux")]
#[path = "ebpf_monitor/linux.rs"]
mod linux;

#[derive(Parser)]
#[command(about = "通过 Aya/eBPF 统计指定用户执行命令的次数和时间")]
struct Args {
    /// 用户真实 UID（通过 id -u 用户名查询）
    #[arg(long)]
    uid: u32,
    /// Linux comm，精确匹配，最多 15 字节，不是路径或完整命令行
    #[arg(long, value_parser = parse_command)]
    command: String,
    /// 编译后的 eBPF ELF；相对路径以当前工作目录为准
    #[arg(
        long,
        default_value = "ebpf/monitor/target/bpfel-unknown-none/release/ebpf-monitor-kernel"
    )]
    object: PathBuf,
}

fn parse_command(value: &str) -> Result<String, String> {
    if value.is_empty()
        || value.len() > 15
        || value.contains('/')
        || value.chars().any(char::is_control)
    {
        return Err("命令名须为 1–15 字节，不得包含路径分隔符或控制字符".into());
    }
    Ok(value.to_owned())
}

#[cfg(any(target_os = "linux", test))]
fn config(args: &Args) -> Config {
    let mut config = Config {
        uid: args.uid,
        command: [0; 16],
    };
    config.command[..args.command.len()].copy_from_slice(args.command.as_bytes());
    config
}

#[cfg(any(target_os = "linux", test))]
fn decode_event(bytes: &[u8]) -> Option<Event> {
    if bytes.len() != size_of::<Event>() {
        return None;
    }
    // SAFETY: 已验证长度；Event 只有整数和字节数组，所有位模式合法；
    // RingBuf 返回的地址不要求满足 Rust Event 的对齐要求。
    Some(unsafe { bytes.as_ptr().cast::<Event>().read_unaligned() })
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(linux::run(args))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = args;
        Err("Aya eBPF 监控需要 Linux，macOS 无法加载该程序".into())
    }
}

fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ebpf_monitor_common::Stats;

    #[test]
    fn validates_kernel_comm_bytes_and_rejects_paths() {
        for invalid in ["", "/bin/ls", "1234567890123456", "ls\n", "中文中文中文"] {
            assert!(parse_command(invalid).is_err());
        }
        assert!(parse_command("123456789012345").is_ok());
        let args =
            Args::try_parse_from(["monitor", "--uid", "1000", "--command", "sleep"]).unwrap();
        let config = config(&args);
        assert_eq!(config.uid, 1000);
        assert_eq!(&config.command, b"sleep\0\0\0\0\0\0\0\0\0\0\0");
    }

    #[test]
    fn decodes_unaligned_events_and_rejects_wrong_abi_size() {
        assert_eq!(size_of::<Config>(), 20);
        assert_eq!(size_of::<Event>(), 48);
        assert_eq!(size_of::<Stats>(), 40);
        let mut bytes = [0u8; 49];
        bytes[1..9].copy_from_slice(&123_u64.to_ne_bytes());
        bytes[17..21].copy_from_slice(&42_u32.to_ne_bytes());
        let event = decode_event(&bytes[1..]).unwrap();
        assert_eq!(event.timestamp_ns, 123);
        assert_eq!(event.pid, 42);
        assert!(decode_event(&bytes).is_none());
        assert!(decode_event(&[]).is_none());
    }
}
