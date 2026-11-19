# 函数指针和返回闭包

普通函数可以作为 `fn` 类型的值传递。不捕获环境的闭包也能转换成函数指针；捕获环境的闭包通常不能。

```rust
fn add_one(value: i32) -> i32 { value + 1 }
fn apply(function: fn(i32) -> i32, value: i32) -> i32 {
    function(value)
}
assert_eq!(apply(add_one, 3), 4);

fn make_adder(offset: i32) -> impl Fn(i32) -> i32 {
    move |value| value + offset
}
assert_eq!(make_adder(5)(3), 8);
```

接收回调时，泛型 `F: Fn(...)` 通常比函数指针更灵活，因为也接受捕获环境的闭包。

闭包的具体类型无法直接写出，可以返回 `impl Fn` 隐藏它。不同分支若返回不同闭包类型，就需要换成统一类型，例如 `Box<dyn Fn(i32) -> i32>`，或者调整设计。

枚举成员构造器也可以像函数一样使用，例如 `.map(Some)` 将每项包装成 Option。

[返回目录](SUMMARY.md)
