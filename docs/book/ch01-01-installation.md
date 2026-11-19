# 安装 Rust

在 [rustup 官网](https://rustup.rs/) 按系统安装。rustup 管理 Rust 工具链，其中包含编译器 `rustc`、项目工具 `cargo` 和标准库。

Linux、macOS 可以在终端执行官网给出的安装命令。Windows 下载并运行安装程序；使用 MSVC 工具链时，还需要按提示安装 Visual Studio 的 C++ 构建工具。

安装后重新打开终端，检查：

```bash
rustc --version
cargo --version
```

如果提示找不到命令，先确认安装是否完成、终端是否重开，再检查 Cargo 的 bin 目录是否在 PATH 中。

常用维护命令：

```bash
rustup update           # 更新工具链
rustup show             # 查看已安装和当前使用的工具链
rustup doc              # 打开本地文档
rustup self uninstall   # 卸载，确实需要时再执行
```

本地英文书可以用 `rustup doc --book` 打开。本项目已经保存了中文改写版，无需联网阅读正文。

[返回目录](SUMMARY.md)
