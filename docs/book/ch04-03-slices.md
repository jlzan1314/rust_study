# 切片：借用连续的一段数据

切片记录一段连续数据的位置和长度，不拥有底层数据。字符串切片是 `&str`，数组或 Vec 的切片是 `&[T]`。

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
let text = String::from("hello Rust");
assert_eq!(first_word(&text), "hello");
let values = [10, 20, 30];
assert_eq!(&values[1..], &[20, 30]);
```

`[..end]` 从头开始，`[start..]` 到末尾，`[..]` 表示全部。范围通常不包含右端。

字符串切片按字节定位，边界必须是 UTF-8 字符边界：

```rust
let text = "你好";
assert_eq!(&text[..3], "你");
assert_eq!(text.get(..1), None); // 不能切开一个汉字。
```

直接写 `&text[..1]` 会 panic。位置不确定时使用 get，它返回 Option。

函数只读文本时通常接收 `&str`，这样 String 和字符串字面量都能传入。只读一段元素时优先接收 `&[T]`，不必限定调用者必须使用 Vec。

[返回目录](SUMMARY.md)
