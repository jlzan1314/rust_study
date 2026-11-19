# 运行第一个程序

新建 `main.rs`：

```rust
fn main() {
    println!("你好，Rust！");
}
```

终端进入文件所在目录，再运行：

```bash
rustc main.rs
./main
```

Windows 下运行生成的 `main.exe`。

`fn` 定义函数，`main` 是程序入口，花括号包住函数体。`println!` 向终端打印一行，末尾的 `!` 表示它是宏。语句通常用分号结束。

`rustc` 先把源码编译成可执行文件，再由你运行。修改源码后需要重新编译。实际项目通常交给 Cargo 管理。

[返回目录](SUMMARY.md)
