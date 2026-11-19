# Rust 怎样组织数据和行为

Rust 没有类继承语法，但能用结构体保存数据、impl 定义行为、模块控制可见性、trait 表达共享能力。

封装的目的，是让外部通过受控方法修改数据，避免破坏内部约定。

```rust
struct Average {
    values: Vec<i32>,
}
impl Average {
    fn new() -> Self { Self { values: Vec::new() } }
    fn add(&mut self, value: i32) { self.values.push(value); }
    fn get(&self) -> Option<f64> {
        if self.values.is_empty() { return None; }
        Some(self.values.iter().map(|&v| f64::from(v)).sum::<f64>()
            / self.values.len() as f64)
    }
}
let mut data = Average::new();
data.add(10);
data.add(20);
assert_eq!(data.get(), Some(15.0));
```

需要复用数据时，用组合：让一个结构体包含另一个。需要统一行为时，用 trait。没有必要先构造一棵继承树，再把所有功能塞进去。

[返回目录](SUMMARY.md)
