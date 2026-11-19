# 读取文本文件

`std::fs::read_to_string` 一次读取整个文件，返回 `Result<String, io::Error>`。

```rust
fn load(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
```

文件不存在、权限不足，或者内容不是有效 UTF-8，都会返回错误。由调用者决定如何提示，不能假定“路径存在就一定能读成功”。

返回的 String 拥有文件内容，之后可以使用 lines 按行处理。相对路径相对于程序运行时的工作目录，不是源码文件所在目录。

小型文本文件适合整文件读取。文件很大时，用 BufReader 配合按行读取，避免把全部内容同时放进内存。

项目中的完整实现见 [io_project](../../src/io_project/mod.rs)。

[返回目录](SUMMARY.md)
