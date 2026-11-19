# 只关心一种情况时，用 if let

只处理 Some，其余不做事时，if let 比写完整 match 更短。

```rust
let limit = Some(10);
if let Some(value) = limit {
    println!("上限是 {value}");
} else {
    println!("没有设置上限");
}
```

`if let 模式 = 表达式` 检查能否匹配，并在分支内使用取出的值。它不检查你是否处理完所有情况；每种情况都重要时，用 match。

需要反复取值直到没有数据时，可以用 `while let`：

```rust
let mut values = vec![1, 2, 3];
while let Some(value) = values.pop() {
    println!("{value}"); // 3、2、1。
}
```

[返回目录](SUMMARY.md)
