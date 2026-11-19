# 单元测试、集成测试和文档测试

三种测试各有用途：

| 类型 | 放哪里 | 验证什么 |
|---|---|---|
| 单元测试 | 源码内 `#[cfg(test)] mod tests` | 一个模块的行为，也能测试其私有实现 |
| 集成测试 | `tests/*.rs` | 把库当外部依赖，验证公开接口协作 |
| 文档测试 | `///` 文档中的 Rust 代码块 | 使用示例是否仍然正确 |

```rust
fn double(value: i32) -> i32 { value * 2 }
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn doubles_positive_value() { assert_eq!(double(3), 6); }
}
```

`cfg(test)` 让测试模块只在测试构建时参与编译。tests 目录中每个顶层 `.rs` 文件通常是独立测试 crate，共用辅助代码可放进 `tests/common/mod.rs`。

二进制入口尽量保持简单，把逻辑放到 lib.rs 组织的库中，集成测试就能直接调用。需要验证参数和退出码时，也可以启动真实二进制。

[返回目录](SUMMARY.md)
