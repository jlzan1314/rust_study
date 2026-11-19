# trait：约定一个类型能做什么

trait 定义能力，具体类型实现能力。函数可以要求“传来的类型会做这件事”，而不限定它叫什么。

```rust
trait Summary {
    fn summarize(&self) -> String;
    fn label(&self) -> &str { "内容" } // 默认实现。
}
struct Article { title: String }
impl Summary for Article {
    fn summarize(&self) -> String { self.title.clone() }
}
fn print_summary(item: &impl Summary) {
    println!("{}：{}", item.label(), item.summarize());
}
let article = Article { title: String::from("学 Rust") };
print_summary(&article);
```

`&impl Summary` 是常用的参数简写。需要表达多个参数必须同一类型，或约束关系复杂时，用命名泛型 `T: Summary`。

多个能力用 `+`，例如 `T: Summary + Clone`。约束太长可移到 where 子句。

返回 `impl Summary` 可以隐藏具体类型，但同一个函数的不同返回分支必须返回同一种具体类型。需要运行时容纳不同类型时，考虑 trait 对象。

实现 trait 通常要求 trait 或目标类型至少有一个定义在当前 crate，不能随意给两个外部定义的项补实现。这样可以避免不同库互相冲突。

[返回目录](SUMMARY.md)
