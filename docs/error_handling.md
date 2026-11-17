# 错误处理学习示例

参考 [Rust 程序设计语言第 9 章](https://rustwiki.org/zh-CN/book/ch09-00-error-handling.html)。
源码与中文注释见 [src/error_handling/mod.rs](../src/error_handling/mod.rs)，独立入口为 [src/bin/error_handling.rs](../src/bin/error_handling.rs)。

```bash
cargo run --bin error_handling
```

示例依次演示：

1. `panic!`、下标越界，以及用 `get` 安全读取元素。
2. `Result<T, E>` 的 `Ok`、`Err`，以及 `unwrap` 与 `expect` 的适用场景。
3. 按 `ErrorKind::NotFound` 区分文件不存在与其他错误。
4. 对比手动 `match` 传播错误、`?` 提前返回错误、`fs::read_to_string` 简化读取。
5. 在返回 `Result` 的 `main` 中使用 `?`。
6. 用私有字段和 `Guess::new` 保证数值在 1 到 100 之间。

文件操作只使用系统临时目录中的独占创建文件，正常结束时删除，不会覆盖项目中的 `hello.txt`。如果临时目录不可写或同名文件已存在，会返回错误。

会触发 panic 的代码保留为注释，可逐个取消注释观察。查看调用栈：

```bash
RUST_BACKTRACE=1 cargo run --bin error_handling
```

用户输入不合法、文件不存在等可预期失败通常应使用 `Result`；调用方违反程序约定时可以使用 `panic!`。`unwrap` 和 `expect` 在遇到 `Err` 时都会 panic，不能代替正常的错误恢复。
