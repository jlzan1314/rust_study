# 用 Cargo 管理项目

Cargo 负责创建项目、下载依赖、编译、运行和测试。

```bash
cargo new hello_rust
cd hello_rust
cargo run
```

常见文件：

| 文件 | 用途 |
|---|---|
| `Cargo.toml` | 项目名称、edition、依赖等配置 |
| `Cargo.lock` | 记录选定的依赖版本，便于复现构建 |
| `src/main.rs` | 二进制程序入口 |
| `src/lib.rs` | 库入口，有库时才需要 |
| `target/` | 编译产物，不应作为源码提交 |

日常使用：

```bash
cargo check          # 快速检查能否编译，不生成最终可执行程序
cargo build          # 编译开发版本
cargo run            # 编译并运行
cargo test           # 运行测试
cargo build --release # 编译经过优化的发布版本
```

在当前学习项目里有多个二进制入口，用 `cargo run --bin smart_pointers` 选择具体示例。

[返回目录](SUMMARY.md)
