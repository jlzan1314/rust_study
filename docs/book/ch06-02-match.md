# match：分别处理每种情况

match 按顺序尝试各个模式，执行第一个匹配的分支。它要求覆盖所有可能，避免漏处理。

```rust
fn add_one(value: Option<i32>) -> Option<i32> {
    match value {
        Some(number) => Some(number + 1),
        None => None,
    }
}
assert_eq!(add_one(Some(2)), Some(3));
assert_eq!(add_one(None), None);
```

`Some(number)` 同时检查情况并取出内部值。分支里多个操作时用花括号，match 也可以产生返回值。

```rust
let number = 3;
let description = match number {
    1 => "一",
    2 | 3 => "二或三",
    other => { println!("其他数字：{other}"); "其他" }
};
assert_eq!(description, "二或三");
```

变量模式会接住剩余任何值；`_` 也能匹配任何值，但不绑定名字。兜底分支应放最后，否则后面的分支永远不会执行。

匹配非 Copy 数据时，按值匹配可能移走内部字段。只想读时，可以匹配 `&value`。

[返回目录](SUMMARY.md)
