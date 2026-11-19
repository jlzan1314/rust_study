# edition：选择语言规则，不是锁定编译器

edition 让 Rust 在兼容旧项目的同时引入新规则。常见版本有 2015、2018、2021、2024，在 Cargo.toml 中设置。

```toml
[package]
name = "example"
version = "0.1.0"
edition = "2024"
```

edition 和 rustc 版本不是一回事。选择 2024 需要足够新的工具链，但不会自动锁死某个编译器版本。不同 edition 的 crate 通常可以互相依赖。

升级时先提交或备份当前代码，再按迁移指南使用 `cargo fix --edition`，检查修改，更新配置，运行测试。命令只能处理它识别的迁移，不能代替验证。

本项目使用 2024 edition。原始中文书来自较早的翻译快照，本改写版按现代稳定版写常用例子。新旧规则差异可查 [Edition Guide](https://doc.rust-lang.org/edition-guide/)。

[返回目录](SUMMARY.md)
