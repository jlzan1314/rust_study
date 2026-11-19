# Deref：让包装类型能像引用一样使用

给类型实现 Deref，就能定义“解引用时看到什么”。deref 方法返回引用，避免直接拿走内部数据。

```rust
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
fn greet(name: &str) { println!("你好，{name}"); }
let number = MyBox(5);
assert_eq!(*number, 5);
let name = MyBox(String::from("Rust"));
greet(&name);
```

greet 需要 `&str`，编译器将 `&MyBox<String>` 逐步转换成 `&String`、`&str`。这是解引用强制转换，不是克隆或转移文本。

教学类型 MyBox 直接保存 T，不像标准库 Box 那样进行堆分配。

共享解引用实现 Deref；可变解引用实现 DerefMut。可变引用可以转成共享引用，共享引用不能凭空变成可变引用。自动转换仍要遵守借用规则。

[返回目录](SUMMARY.md)
