# trait 的几种进阶用法

## 关联类型

当一个实现对应一个固定类型时，用关联类型。例如 Iterator 的 Item 说明这个迭代器每次产生什么。

```rust
trait Source {
    type Item;
    fn get(&self) -> Self::Item;
}
struct Number;
impl Source for Number {
    type Item = i32;
    fn get(&self) -> i32 { 42 }
}
assert_eq!(Number.get(), 42);
```

泛型参数通常允许同一类型针对不同参数实现多次；同一个确定的 trait 实现中，关联类型只有一个选择。

## 默认参数和运算符

`Add<Rhs = Self>` 的 Rhs 有默认值，因此通常可以定义两个同类值相加。实现 Add 能定义 `+` 的行为，但不能发明新的运算符。

## 同名方法和父 trait

同名方法有歧义时，用 `<Type as Trait>::method(...)` 明确指定。`trait Outline: Display` 表示实现 Outline 前必须具备 Display 能力。

## newtype

不能直接给两个外部定义的项补实现时，可以把外部类型包进自己的结构体，例如 `struct Names(Vec<String>);`，再给 Names 实现 Display。这样还能限制接口、区分业务含义，不必把内部类型的所有方法都暴露出去。

[返回目录](SUMMARY.md)
