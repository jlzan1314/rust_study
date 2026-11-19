# 用结构体表达一个矩形

把宽和高作为独立参数，容易传错；放进 Rectangle，代码就能直接表达“这是一个矩形”。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
let rect = Rectangle { width: 30, height: 50 };
assert_eq!(area(&rect), 1500);
println!("{rect:?}");
```

area 只读数据，所以接收引用，不拿走矩形。`#[derive(Debug)]` 自动生成调试输出能力；`{:?}` 简单打印，`{:#?}` 分行打印。

`dbg!(&rect)` 适合临时观察值和位置，输出到标准错误。传入引用可以继续使用 rect；直接传入非 Copy 值会取得它的所有权。

[返回目录](SUMMARY.md)
