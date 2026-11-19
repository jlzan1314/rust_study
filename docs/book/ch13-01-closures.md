# 闭包：可以使用周围变量的小函数

闭包用竖线写参数，类型通常可以省略。它可以直接使用定义位置附近的变量。

```rust
let offset = 5;
let add = |value| value + offset;
assert_eq!(add(3), 8);
```

编译器按闭包实际行为决定如何捕获变量：共享借用、可变借用，或移走所有权。加 move 会按值捕获，但不代表只能调用一次。

| trait | 闭包允许的行为 |
|---|---|
| `Fn` | 不需要修改或移出捕获值，可反复调用 |
| `FnMut` | 可以修改捕获值，不移出捕获值 |
| `FnOnce` | 可能移出捕获值，只保证能调用一次 |

所有闭包都实现 FnOnce，实现 Fn 的闭包也实现 FnMut。函数接收闭包时，选择满足需要的最宽松约束；只调用一次，FnOnce 往往就够了。

```rust
let text = String::from("Rust");
let size = move || text.len();
assert_eq!(size(), 4);
assert_eq!(size(), 4); // text 被捕获，但没有从闭包中移出。
```

每个闭包都有自己的类型，同一闭包参数的类型一旦确定，不能换另一种类型调用。

缓存闭包结果时，必须按输入区分缓存，不能用一个 Option 缓存所有参数的结果。闭包依赖外部可变状态时，还要考虑缓存何时失效。项目中的 HashMap 缓存例子见[闭包源码](../../src/functional/closures.rs)。

[返回目录](SUMMARY.md)
