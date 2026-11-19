# 关键字按用途记

先认得常用关键字，不必一次背全。

| 用途 | 常见关键字 |
|---|---|
| 变量与值 | let、mut、const、static、true、false |
| 函数与类型 | fn、struct、enum、type、trait、impl、dyn |
| 条件和循环 | if、else、match、loop、while、for、in、break、continue |
| 返回和移动 | return、move、ref |
| 模块与可见性 | mod、use、pub、crate、super、self、Self |
| 泛型和约束 | where、as |
| 异步与底层接口 | async、await、unsafe、extern |

self 表示当前值或当前模块路径，Self 表示当前实现中的类型，两者大小写不同。ref 出现在模式中，用于绑定引用；`&` 还可以用于创建引用和匹配引用。

某些词为将来保留，不能直接当普通变量名；edition 也会增加关键字。需要引用与关键字同名的既有接口时，可以使用原始标识符，例如 `r#type`。

完整列表以 [Rust Reference](https://doc.rust-lang.org/reference/keywords.html) 为准。

[返回目录](SUMMARY.md)
