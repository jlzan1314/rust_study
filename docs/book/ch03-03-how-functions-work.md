# 函数：输入、处理、返回值

函数用 `fn` 定义。参数必须标明类型，有返回值时用 `->` 写出返回类型。

```rust
fn add(left: i32, right: i32) -> i32 {
    left + right
}
assert_eq!(add(2, 3), 5);
```

函数体最后一个没有分号的表达式就是返回值。写成 `left + right;` 会丢弃结果，整个函数体得到 `()`，与声明的 i32 不符。

语句执行操作，表达式产生值。代码块本身也可以是表达式：

```rust
let value = {
    let base = 2;
    base * 3
};
assert_eq!(value, 6);
```

`return value;` 可以提前返回。没有声明返回类型的函数通常返回单元类型 `()`。

[返回目录](SUMMARY.md)
