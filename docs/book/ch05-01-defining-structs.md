# 结构体：把相关数据放在一起

一个用户包含姓名和状态，用结构体比两个不相关的变量更清楚。

```rust
struct User {
    name: String,
    active: bool,
}
let mut user = User { name: String::from("小明"), active: true };
user.active = false;
assert_eq!(user.name, "小明");
assert!(!user.active);
```

用 `对象.字段` 访问。需要修改字段时，整个变量绑定要声明 mut，不能只在字段定义上加 mut。

字段名和变量名相同，可以写 `User { name, active }`。用 `..old` 从旧值补齐剩余字段时，非 Copy 字段可能被移动，不能假定 old 仍能完整使用。

元组结构体没有字段名，例如 `struct Point(i32, i32);`。它能给一组值单独的类型，避免把不同概念混用。没有字段的 `struct Marker;` 适合只表达类型身份。

结构体保存 String 时拥有文本。保存 `&str` 时只借用文本，需要说明生命周期；入门时先用 String。

[返回目录](SUMMARY.md)
