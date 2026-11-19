# 模块和可见性

用 mod 将代码分组。默认情况下，模块中的项对父模块私有；想让外部访问，使用 pub。

```rust
mod restaurant {
    pub mod service {
        pub fn order() -> &'static str { "已下单" }
    }
    fn internal_note() {} // 外部不能调用。
}
assert_eq!(restaurant::service::order(), "已下单");
```

访问一条路径时，沿途需要经过的模块和目标项都必须对调用位置可见。子模块可以访问祖先模块的私有项，外部代码不能随意访问子模块内部。

结构体公开不代表字段公开：`pub struct User` 的字段仍要分别决定是否 pub。枚举公开后，枚举成员也公开。

常见限制：`pub(crate)` 只向当前 crate 公开，`pub(super)` 向父模块公开。尽量公开需要的接口，把实现细节留在模块内部。

[返回目录](SUMMARY.md)
