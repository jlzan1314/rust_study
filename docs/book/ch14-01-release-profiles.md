# 开发版和发布版怎么选

`cargo build` 默认使用 dev 配置，方便调试；`cargo build --release` 使用 release 配置，更重视运行速度，通常编译更慢。

在 Cargo.toml 中按需调整：

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

opt-level 控制优化程度，不应假定数值越大在所有场景都越好。优化体积等目标还有其他设置，见 [Cargo 构建配置](https://doc.rust-lang.org/cargo/reference/profiles.html)。

调试版通常开启整数溢出检查，发布版默认设置不同，不要让正确性依赖某个 profile。需要确定行为时，用 checked、wrapping 或 saturating 系列方法。

工作空间的 profile 通常应在根 Cargo.toml 配置，成员中的设置不会按独立项目方式生效。

[返回目录](SUMMARY.md)
