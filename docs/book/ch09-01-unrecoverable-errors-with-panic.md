# panic：程序无法继续时停止

panic 表示当前执行无法正常继续，例如内部状态被破坏、直接索引越界。主动触发使用 `panic!("说明")`。

```rust,should_panic
let values = vec![1, 2, 3];
let index = 99;
println!("{}", values[index]); // 越界，触发 panic。
```

默认情况下，panic 会展开当前线程的栈并清理局部变量。也可以配置成直接终止进程；这时不会进行栈展开。panic 不一定让整个多线程进程立即退出，取决于发生在哪个线程以及如何处理。

排查时查看报错位置，必要时启用回溯：

```bash
RUST_BACKTRACE=1 cargo run
```

不要只盯着标准库的最后一行，先找回溯中属于自己项目的调用位置。可以预期的失败，例如文件不存在、用户输错数字，通常应该返回 Result，而不是直接 panic。

[返回目录](SUMMARY.md)
