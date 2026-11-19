# 常用工具：格式化、检查、文档

```bash
rustup component add rustfmt clippy
cargo fmt             # 统一格式
cargo fmt --check     # 只检查，不改文件
cargo clippy          # 发现常见问题和改进建议
cargo check           # 检查能否编译
cargo test            # 验证行为
cargo doc --no-deps --open # 生成并打开项目 API 文档
```

rust-analyzer 提供类型提示、补全、跳转和诊断，编辑器通常可以安装对应扩展。它帮助写代码，但不替代实际编译和测试。

Clippy 建议需要结合代码目的判断，不要为消除一条提示而改坏接口。持续集成中可以按项目需要将警告视为错误，但应先统一工具链版本和检查范围。

定位运行时问题还可以用日志、调试器和性能分析工具，先拿到证据，再决定怎么改。

[返回目录](SUMMARY.md)
