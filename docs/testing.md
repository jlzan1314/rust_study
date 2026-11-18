# 自动化测试学习示例

参考 [Rust 程序设计语言第 11 章](https://rustwiki.org/zh-CN/book/ch11-00-testing.html)。本章通过 `cargo test` 执行，不需要额外的二进制入口。

| 位置 | 内容 |
|---|---|
| [src/testing/mod.rs](../src/testing/mod.rs) | 被测代码、单元测试、文档测试与中文注释 |
| [tests/testing_examples.rs](../tests/testing_examples.rs) | 以外部使用者身份调用公开 API 的集成测试 |
| [tests/testing_common/mod.rs](../tests/testing_common/mod.rs) | 集成测试共享的辅助函数 |

## 编写测试

`#[test]` 标记测试函数，`#[cfg(test)]` 让单元测试模块只在测试构建中编译。普通测试正常返回即通过，断言失败或未预期的 panic 会失败。

示例包含 `assert!`、`assert_eq!`、`assert_ne!`、自定义失败消息、`#[should_panic]`、匹配 panic 消息子串，以及通过 `Result` 和 `?` 返回测试错误。复用第 9 章的 `Guess` 验证非法输入和合法边界。

## 运行测试

```bash
# 只运行本章单元测试
cargo test --lib testing::tests

# 按名字子串筛选，本例运行两个 add_two 测试
cargo test --lib testing::tests::add_two

# 精确匹配完整测试名
cargo test --lib testing::tests::greeting_contains_name -- --exact

# 显示成功测试的输出
cargo test --lib testing::tests -- --show-output

# 串行运行；默认测试可以并行执行，不应依赖执行顺序
cargo test --lib testing::tests -- --test-threads=1

# 只运行标记了 ignore 的本章测试
cargo test --lib testing::tests -- --ignored

# 运行本章全部单元测试，包括 ignore 项
cargo test --lib testing::tests -- --include-ignored

# 只运行本章集成测试入口
cargo test --test testing_examples

# 运行本章文档测试
cargo test --doc testing::add_two

# 运行整个项目的单元测试、集成测试和文档测试
cargo test
```

`--` 前是 Cargo 的参数，后面是测试运行器的参数。默认忽略一项用于教学的 `#[ignore]` 测试，它没有实际耗时操作；故意失败的代码保留为注释。

## 测试组织

单元测试是源码中的子模块，可以访问父模块的私有实现。集成测试文件作为独立 crate 编译，通过 `rust_study::...` 访问公开接口。辅助文件放在子目录的 `mod.rs` 中，通过 `mod` 引入，避免成为独立测试入口。

文档测试是文档注释中的 Rust 代码块，普通 `cargo test` 会执行它；`cargo test --all-targets` 不包含文档测试，需要另外执行 `cargo test --doc`。
