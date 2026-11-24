# Aya 原生 eBPF 用户命令监控

本示例全部使用 Rust：内核端通过 `aya-ebpf` 监听成功 exec 和进程退出，用户端通过 `aya` 加载 ELF、设置筛选条件、读取 RingBuf 事件，并在 Ctrl-C 或 SIGTERM 时读取内核统计。运行时不需要 bpftrace、BCC、clang 或 libbpf。

## 代码结构

| 位置 | 职责 |
|---|---|
| `ebpf/monitor/src/main.rs` | eBPF 程序、UID/comm 筛选、计时、Map 与 RingBuf |
| `ebpf/common/src/lib.rs` | 两端共享的配置、事件和统计 ABI，`no_std` |
| `src/bin/ebpf_monitor.rs` | 参数解析与入口，非 Linux 平台提示 |
| `src/bin/ebpf_monitor/linux.rs` | Aya 加载、挂载、事件输出、信号处理与汇总 |

内核 Map 包括：`CONFIG` 保存筛选条件；`STARTED` 按 TGID 保存最近一次匹配的 exec；`STATS` 按 CPU 保存次数、累计耗时及错误数；`EVENTS` 是 1 MiB RingBuf。汇总直接读取 STATS，不依赖用户端收到的事件数量。

## 构建

运行需要 Linux 5.8 或更新版本（RingBuf 要求）、tracefs 和 BPF 权限。建议在 Linux 上使用 root 运行；容器还需要宿主机允许 BPF 和 tracepoint 访问。macOS 可以构建用户端和 eBPF ELF，但不能加载运行。

项目使用 Aya 0.13.1 / aya-ebpf 0.1.1，用户端需要 Rust 1.85+（edition 2024）。eBPF 构建需要带 rust-src 的 nightly 和 bpf-linker。以下固定工具版本，避免 linker 与 Rust 所用 LLVM 不匹配：

```bash
rustup toolchain install nightly-2026-01-15 --component rust-src
cargo +nightly-2026-01-15 install bpf-linker --version 0.9.15 --locked

# 必须切换目录，使 monitor/.cargo/config.toml 的 BPF target/linker 配置生效。
cd ebpf/monitor
cargo +nightly-2026-01-15 build --release --locked -Z build-std=core
cd ../..

# 用户端用 stable 编译；两部分独立构建，其他示例不需要 BPF 工具链。
cargo build --release --locked --bin ebpf_monitor
```

若在 macOS 上使用 Homebrew Rust，bpf-linker 可能找不到 LLVM 动态库；需要安装与编译器 LLVM 主版本一致的 LLVM，并在构建 eBPF 时设置 `DYLD_FALLBACK_LIBRARY_PATH` 指向其 `lib` 目录。rustup nightly 通常自带所需动态库。

默认生成小端 BPF ELF，适用于常见的 x86_64 / aarch64 Linux。大端机器需改用 `bpfeb-unknown-none`，并通过 `--object` 指向相应产物。分发到目标 Linux 时携带用户端二进制和对应 ELF，目标机器无需安装 Rust。

## 运行与验收

从仓库根目录运行：

```bash
sudo ./target/release/ebpf_monitor --uid "$(id -u alice)" --command sleep

# 自定义 ELF 路径（例如分发到另一台 Linux）
sudo ./target/release/ebpf_monitor --uid 1000 --command sleep \
  --object /opt/ebpf/ebpf-monitor-kernel

# 另一个终端触发三次成功 exec；建议选择没有其他 sleep 任务的测试用户
sudo -u alice sh -c 'sleep 1; sleep 2; sleep 1'
```

会输出类似以下事件（`unix_time` 为 Unix 秒，含小数）：

```text
EXEC unix_time=1795500000.000000000 uid=1000 pid=1234 comm=sleep
EXIT unix_time=1795500001.001000000 uid=1000 pid=1234 comm=sleep elapsed_ms=1001.000
```

三个命令退出后按 Ctrl-C，预期汇总 `executions=3 completed=3 total_ms=约4000 dropped_events=0 tracking_errors=0`。并发命令的耗时相加可能超过监控经过的时间。

`dropped_events` 表示 RingBuf 满导致的明细丢失，内核次数和耗时仍会累计。`tracking_errors` 表示读取 comm 或保存/删除计时记录失败，例如同时监控超过 16,384 个进程；非零时完成数与耗时可能不完整。明细每 50ms 批量读取并刷新输出。

## 统计口径与边界

- `--uid` 是 exec 时的真实 UID，通过 `id -u` 从用户名转换。容器 UID 映射需按宿主机所见 UID 设置。
- `--command` 精确匹配 Linux `comm`（最多 15 字节），例如 `sleep`；不能写 `/usr/bin/sleep` 或 `sleep 1`。同名可执行文件会合并。解释器脚本的 comm 由内核决定，可用 `ps -o pid,comm,args` 确认。
- 只统计探针挂载后成功的 exec；失败执行、既有进程、shell 内建命令都不计数。两种探针的挂载/卸载存在短暂边界窗口。
- 耗时从成功 exec 到线程组主线程退出，包含等待和睡眠，不是 CPU 时间。主线程提前退出、其他线程继续运行时，不等于整个线程组的生存时间。
- 同一 PID 再次 exec 会清除旧计时；新程序匹配则重新计时。被替换的程序计入执行次数，不计入已完成耗时。退出时即使 UID 或 comm 变化，也按原记录结算。
- 停止时仍在运行的命令不计入完成数和耗时；先卸载探针，再排空事件并读取统计。强制 SIGKILL 无法保证输出汇总。
- 单次耗时使用内核 CLOCK_MONOTONIC，不含机器挂起时间。事件 Unix 时间通过启动时的墙钟偏移换算，运行期间调整系统时间或机器挂起后可能偏离实际墙钟，耗时不受墙钟调整影响。
- 本示例不持久化历史事件。内核加载和 verifier 验收需要在目标 Linux 执行。

## 本地检查

```bash
cargo test --bin ebpf_monitor
cargo run --bin ebpf_monitor -- --help
```

测试覆盖 comm 字节长度、配置编码、共享 ABI 大小及未对齐事件解码。这些检查可在 macOS 执行，不能替代 Linux 上的探针挂载与上述三次执行验收。
