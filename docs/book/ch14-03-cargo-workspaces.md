# 工作空间：一起管理多个 Package

当一个项目包含多个相关库或工具时，可以用 workspace 统一管理。成员通常共享根目录的 Cargo.lock 和 target。

根 Cargo.toml 可以只写：

```toml
[workspace]
members = ["app", "math"]
resolver = "3"
```

resolver 3 适用于较新的 Cargo；本项目使用 Rust 2024 工具链。两个成员各有自己的 Cargo.toml，app 使用本地 math 库时声明路径依赖：

```toml
[dependencies]
math = { path = "../math" }
```

常用命令：`cargo test --workspace` 测试全部成员，`cargo run -p app` 运行某个 Package，`cargo test -p math` 测试指定成员。

成员不会仅仅因为处于同一个工作空间就自动互相依赖，仍然需要声明依赖。统一版本或依赖设置可以放在根 workspace 配置中，再由成员继承。

[返回目录](SUMMARY.md)
