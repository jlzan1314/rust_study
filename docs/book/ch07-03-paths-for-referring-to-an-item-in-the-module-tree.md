# 路径：告诉 Rust 去哪里找名字

`::` 连接模块和项。常见路径起点：

| 写法 | 从哪里开始找 |
|---|---|
| `crate::` | 当前 crate 的根 |
| `self::` | 当前模块 |
| `super::` | 父模块 |
| 依赖 crate 名 | 对应的外部 crate |

```rust
fn helper() -> i32 { 3 }
mod child {
    pub fn run() -> i32 { super::helper() }
}
fn main() {
    assert_eq!(child::run(), 3);
}
```

`src/main.rs` 和 `src/lib.rs` 是不同 crate 的入口。main.rs 里的 `crate::` 指向二进制自己，不会自动指向库；访问本项目的库要使用 `rust_study::...`。

移动模块位置后，相对路径可能要调整。模块层次复杂时，从 crate 根开始的路径更容易定位。

[返回目录](SUMMARY.md)
