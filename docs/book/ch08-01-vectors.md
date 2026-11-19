# Vec：长度可以变化的数组

`Vec<T>` 保存同一种类型的多个值，长度可以增加或减少。

```rust
let mut values = vec![1, 2, 3];
values.push(4);
assert_eq!(values.pop(), Some(4));
assert_eq!(values.get(1), Some(&2));
assert_eq!(values.get(9), None);
for value in &mut values { *value += 1; }
assert_eq!(values, vec![2, 3, 4]);
```

`Vec::new()` 创建空 Vec。编译器不知道元素类型时，要写成 `let values: Vec<i32> = Vec::new();`。

`values[index]` 越界会 panic，get 返回 Option。get 得到的是元素引用，不会复制整个元素。

## 三种遍历

| 方法 | 得到什么 | 影响 |
|---|---|---|
| `iter()` | `&T` | 只借用，原集合仍在 |
| `iter_mut()` | `&mut T` | 可修改原元素 |
| `into_iter()` | `T` | 直接对 Vec 调用会消耗它 |

```rust
let values = vec![1, 2, 3];
let result: Vec<_> = values.iter().map(|&v| v + 1).collect();
assert_eq!(values, vec![1, 2, 3]);
assert_eq!(result, vec![2, 3, 4]);
```

这里的 `|&v|` 从引用中复制整数，计算的是新值，不修改 values。对引用调用 into_iter 则是借用，例如 `(&values).into_iter()`，不能一概理解成消耗集合。

push 可能重新分配存储空间。因此元素引用后面还要用时，不能同时 push。要保存不同类型的内容，可以先定义枚举，再用 `Vec<枚举类型>`。

更多方法见[集合速查](../collections.md)。

[返回目录](SUMMARY.md)
