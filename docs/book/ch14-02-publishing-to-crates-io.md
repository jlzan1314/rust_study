# 发布库之前，先让别人能用

一个可用的库至少要说明：做什么、如何安装、最小例子、可能失败的情况和许可证。

`///` 文档中的 Rust 代码块可以被 cargo test 当作文档测试运行。`//!` 说明当前模块或 crate，`pub use` 可以为使用者提供稳定的公开路径。

发布前检查 Cargo.toml 的名称、版本、描述和许可等信息，确认包里没有私密文件，再检查实际打包内容：

```bash
cargo test
cargo doc --no-deps
cargo package --list
cargo publish --dry-run
```

真正发布还需要 crates.io 账号及凭据，并执行 cargo publish。上面的 dry-run 用于本地验证，不会上传包。

同一个版本号通常不能重复发布，更新代码后要递增版本并考虑兼容性。yank 用于阻止新的依赖解析选中问题版本，既有锁文件可能仍会使用它；它不等于从网络上删除代码。

具体账号认证和发布要求以 [crates.io 当前文档](https://doc.rust-lang.org/cargo/reference/publishing.html) 为准。

[返回目录](SUMMARY.md)
