# cargo install：安装命令行工具

cargo install 编译并安装提供二进制程序的包，安装后可以直接在终端运行。它与在项目里声明库依赖是两回事。

```bash
cargo install ripgrep --locked
rg --version
```

默认安装位置通常是 Cargo 主目录下的 bin，需要在 PATH 中。`--locked` 要求使用包所带的锁文件，适合按作者锁定的依赖构建。

用 `cargo install --list` 查看已安装工具，`cargo uninstall 包名` 卸载。库依赖用 cargo add 或 Cargo.toml 管理，不要为了引用一个库先 cargo install。

[返回目录](SUMMARY.md)
