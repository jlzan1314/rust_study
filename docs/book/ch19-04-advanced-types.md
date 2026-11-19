# 类型别名、never 和动态大小

## 类型别名不创建新类型

```rust
type Kilometers = i32;
let distance: Kilometers = 5;
let total: i32 = distance + 2;
assert_eq!(total, 7);
```

Kilometers 只是 i32 的另一个名字，无法阻止米和公里混用。需要类型隔离时，使用 `struct Kilometers(i32)` 这样的 newtype。

## ! 表示不会正常返回

panic、无限循环、return、continue 等不会给当前表达式提供正常结果。never 类型 `!` 可以帮助不同控制流分支统一类型，例如 match 的一个分支返回值，另一个分支直接 panic。

## 有些类型大小不能在编译时确定

str、切片 `[T]`、trait 对象 `dyn Trait` 属于动态大小类型。通常把它们放在引用或 Box 等指针后使用，例如 `&str`、`&[T]`、`Box<dyn Trait>`。

泛型参数默认带有 Sized 约束。写 `T: ?Sized` 表示允许 T 的大小不能静态确定，此时函数通常通过 `&T` 等方式访问。`?Sized` 是放宽默认约束，不是要求类型一定动态大小。

[返回目录](SUMMARY.md)
