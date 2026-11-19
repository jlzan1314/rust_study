# 把参数、业务逻辑和程序入口分开

main 只负责三件事：拿到输入、调用业务逻辑、将结果转换为输出和退出状态。

建议拆成：

- `Config::build`：检查参数，生成配置，失败就返回错误。
- `run(config)`：读文件并搜索，返回 Result。
- main：显示错误，并决定进程是否以失败状态退出。

```rust
fn parse_limit(text: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let limit = text.parse()?;
    Ok(limit)
}
assert_eq!(parse_limit("10").unwrap(), 10);
assert!(parse_limit("abc").is_err());
```

`Box<dyn Error>` 可以统一返回不同的错误类型，适合小型应用。库需要调用者区分错误时，可以定义自己的错误枚举。

不要让每层都打印同一条错误。底层返回错误，上层补充上下文，最外层负责展示。

`process::exit` 会立即结束进程，不运行栈上局部变量的析构。需要正常清理时，可以从 main 返回 ExitCode 或 Result。

[返回目录](SUMMARY.md)
