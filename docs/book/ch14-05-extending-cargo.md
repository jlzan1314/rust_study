# Cargo 扩展命令

如果 PATH 中存在名为 `cargo-example` 的可执行文件，就可以通过 `cargo example` 调用。很多 Cargo 子命令就是独立工具。

`cargo --list` 查看可用命令。需要某个工具时，先确认来源和用途，再按它的安装说明操作。

扩展命令不会改变 Rust 语法，只是在 Cargo 的命令形式下运行一个外部程序。日常常见的 fmt、clippy 通常通过 rustup 组件安装。

[返回目录](SUMMARY.md)
