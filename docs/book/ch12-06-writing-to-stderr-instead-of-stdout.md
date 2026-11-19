# 正常结果和错误分开输出

正常结果用 println，错误和诊断用 eprintln。这样用户可以单独保存搜索结果：

```bash
cargo run --bin minigrep -- Rust docs/poem.txt > result.txt
```

`>` 重定向标准输出，错误仍显示在终端。需要单独保存错误时，可以使用 `2> error.txt`。

```rust,no_run
use std::process::ExitCode;
fn main() -> ExitCode {
    eprintln!("用法：minigrep <关键词> <文件路径>");
    ExitCode::from(2)
}
```

退出码 0 通常表示成功，非 0 表示失败，便于 shell 和其他程序判断。本项目将“没有匹配”当作成功且输出为空；参数错误、读文件失败才作为失败返回。

上面是演示失败退出的独立入口，不要把它当成搜索程序的完整实现。完整项目见[项目文档](../io_project.md)。

[返回目录](SUMMARY.md)
