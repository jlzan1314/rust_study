# 泛型、trait 和生命周期

参考 [Rust 程序设计语言第 10 章](https://rustwiki.org/zh-CN/book/ch10-00-generics.html)，示例附中文注释，按书中三节组织。

```bash
cargo run --bin generics
```

| 章节 | 源码 | 示例内容 |
|---|---|---|
| 10.1 泛型 | [types.rs](../src/generics/types.rs) | largest 函数、Point、多个类型参数、泛型方法、特定类型的方法、泛型枚举 |
| 10.2 trait | [traits.rs](../src/generics/traits.rs) | Summary、默认实现、impl Trait、trait bound、where、多重约束、条件实现 |
| 10.3 生命周期 | [lifetimes.rs](../src/generics/lifetimes.rs) | longest、借用范围、保存引用的结构体、省略规则、static、综合泛型示例 |

入口为 [src/bin/generics.rs](../src/bin/generics.rs)，只调用本章模块，不会运行其他章节。

`largest` 要求非空输入；为配合书中的 `Copy` 版本，示例使用整数和字符。`PartialOrd` 允许不可比较的值，不能据此认为包含 NaN 的浮点切片必然得到数学意义上的最大值。

生命周期标注描述引用之间的关系，不会延长数据寿命。无法通过编译的类型混用、悬垂引用示例保留为注释，可逐个取消注释查看编译器提示。`longest` 使用字节长度比较，与字符数量不同。
