# 泛型：同一份逻辑处理多种类型

当函数只依赖某些能力，不依赖具体类型时，可以把类型写成参数 T。

```rust
fn largest<T: PartialOrd>(values: &[T]) -> Option<&T> {
    let mut largest = values.first()?;
    for value in &values[1..] {
        if value > largest { largest = value; }
    }
    Some(largest)
}
assert_eq!(largest(&[2, 5, 1]), Some(&5));
assert_eq!(largest::<i32>(&[]), None);
```

`PartialOrd` 表示能做大小比较。函数返回引用，因此不要求 T 能复制；空切片返回 None。对浮点数要另行考虑 NaN，因为它不满足普通的全序比较。

结构体和枚举也能使用泛型：`struct Point<T> { x: T, y: T }`。两个字段写成 T，就必须是同一种类型；要允许不同类型，可以用 `Point<T, U>`。

泛型方法放在 `impl<T> Point<T>` 中。只针对特定类型的方法，可以写在 `impl Point<f64>` 中。

编译器通常会为实际使用的类型生成对应代码，叫单态化。它不要求运行时先查询 T 是什么，但可能增加编译时间和生成代码体积。

[返回目录](SUMMARY.md)
