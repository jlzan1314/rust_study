# 枚举：一个值有几种可能

结构体把多个字段放在一起；枚举表示“当前是哪一种情况”。每个枚举成员还能保存不同的数据。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
}
let message = Message::Text(String::from("你好"));
match message {
    Message::Quit => println!("退出"),
    Message::Move { x, y } => println!("移动到 {x}, {y}"),
    Message::Text(text) => println!("{text}"),
}
```

这样不会出现“消息类型是退出，但还塞着移动坐标”的无效组合。枚举也可以在 impl 中定义方法。

## Option：有值或没值

标准库定义 `Option<T>`：`Some(T)` 表示有值，`None` 表示没有。它与 T 是不同的类型，必须先处理可能为空的情况。

```rust
let value: Option<i32> = Some(3);
assert_eq!(value.unwrap_or(0), 3);
let missing: Option<i32> = None;
assert_eq!(missing.unwrap_or(0), 0);
```

不要到处用 unwrap 跳过空值处理；None 上调用 unwrap 会 panic。

[返回目录](SUMMARY.md)
