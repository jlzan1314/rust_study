# 方法：把操作放到类型上

在 `impl 类型名` 中定义方法。`&self` 表示只读访问当前对象，`&mut self` 表示修改，`self` 表示取得所有权。

```rust
struct Rectangle { width: u32, height: u32 }
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}
let outer = Rectangle { width: 10, height: 20 };
let inner = Rectangle::square(5);
assert_eq!(outer.area(), 200);
assert!(outer.can_hold(&inner));
```

没有 self 参数的函数叫关联函数，用 `类型::函数` 调用。square 就是一个构造函数，但 Rust 不强制构造函数必须叫 new。

调用 `rect.area()` 时，编译器会按需要自动借用或解引用，因此通常不用手动写 `(&rect).area()`。这种便利不会放宽借用规则。

一个类型可以写多个 impl 块，用来按功能组织方法。

[返回目录](SUMMARY.md)
