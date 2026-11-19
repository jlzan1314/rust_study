# 运行测试和筛选测试

常用命令：

```bash
cargo test                      # 运行测试，包括文档测试
cargo test smart_pointers       # 按测试路径中的文字筛选
cargo test --lib                 # 只运行库的单元测试
cargo test --test functional_examples # 指定集成测试文件
cargo test -- --show-output      # 显示成功测试捕获的输出
cargo test -- --test-threads=1   # 串行运行
cargo test -- --ignored          # 运行标记为 ignored 的测试
```

`--` 前面的参数交给 Cargo，后面的参数交给测试程序。名称过滤通常是包含匹配，需要精确匹配时加 `--exact` 并提供完整测试路径。

测试默认可能并行运行，不要让多个测试争用同一个文件名、端口或全局环境变量。优先给每个测试独立的数据和资源；确实共享时，再考虑串行运行。

耗时或需要特殊环境的测试可标注 `#[ignore]`，默认跳过。跳过的测试不代表已经验证通过。

[返回目录](SUMMARY.md)
