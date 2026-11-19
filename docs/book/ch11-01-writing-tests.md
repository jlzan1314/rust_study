# 写测试：给输入，检查结果

测试是带 `#[test]` 的函数。正常结束表示通过，panic 表示失败。

```rust
fn add(a: i32, b: i32) -> i32 { a + b }
#[test]
fn adds_two_numbers() {
    assert_eq!(add(2, 3), 5);
}
```

| 写法 | 检查什么 |
|---|---|
| `assert!(condition)` | 条件为真 |
| `assert_eq!(left, right)` | 两边相等，失败时显示两边值 |
| `assert_ne!(left, right)` | 两边不相等 |
| `#[should_panic]` | 测试应该触发 panic |

断言可以附加说明：`assert!(value > 0, "应为正数，实际为 {value}");`。错误信息应帮助定位问题。

测试预期 panic 时，可以加 `#[should_panic(expected = "错误片段")]`，避免其他原因的 panic 被误当成成功。

测试函数也能返回 `Result<(), E>`，这样可以在准备数据时使用 `?`；成功时返回 Ok，失败时返回 Err。不要给这种测试再加 should_panic。

测试要检查外部行为、边界和失败情况，不要只复述函数内部怎么写。

[返回目录](SUMMARY.md)
