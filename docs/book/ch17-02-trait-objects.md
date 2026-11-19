# trait 对象：同一个列表，放不同类型

泛型适合在编译时确定类型。若列表里要放多种具体类型，但它们都能执行同一种操作，可以用 trait 对象。

```rust
trait Draw { fn draw(&self) -> String; }
struct Button;
struct Label;
impl Draw for Button { fn draw(&self) -> String { "按钮".into() } }
impl Draw for Label { fn draw(&self) -> String { "文字".into() } }
let widgets: Vec<Box<dyn Draw>> = vec![Box::new(Button), Box::new(Label)];
assert_eq!(widgets[0].draw(), "按钮");
assert_eq!(widgets[1].draw(), "文字");
```

`dyn Draw` 表示具体类型在运行时才能确定，通过虚表找到方法，叫动态分发。Box 提供统一大小的拥有型句柄；不需要所有权时也可以用 `&dyn Draw`。

与泛型相比，trait 对象多一层间接调用，但能在运行时组合不同类型。泛型通常使用静态分发，各有用途。

trait 必须满足 dyn 兼容规则，旧资料也称对象安全。常见限制是：可通过对象调用的方法不能有自己的类型参数，不能直接返回未知大小的 Self。需要这类方法时，可以为它加 `where Self: Sized`，让它不参与对象调用。完整规则见 [Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)。

[返回目录](SUMMARY.md)
