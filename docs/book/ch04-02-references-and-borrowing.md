# 引用：使用数据，但不拿走它

传 `&value` 给函数，表示暂时借给它。函数用完后，原来的所有者还能继续使用。

```rust
fn length(text: &str) -> usize {
    text.len()
}
let text = String::from("Rust");
assert_eq!(length(&text), 4);
println!("{text}"); // text 没有被移走。
```

`&T` 是共享引用，通常只能读取。需要修改时使用 `&mut T`：

```rust
fn append_mark(text: &mut String) {
    text.push('!');
}
let mut text = String::from("Rust");
append_mark(&mut text);
assert_eq!(text, "Rust!");
```

## 借用规则

同一份数据，在借用仍会被使用的期间：

- 可以有多个共享引用。
- 或者有一个独占的可变引用。
- 引用不能比它指向的数据活得更久。

```rust,compile_fail
let mut text = String::from("Rust");
let reader = &text;
text.push('!'); // reader 后面还要用，此时不能修改 text。
println!("{reader}");
```

普通引用的借用常在最后一次使用后就结束，不一定要等到花括号结束。RefCell 的借用守卫不同，见第 15 章。

不能返回局部变量的引用：局部变量在函数结束时销毁，引用就会失效。应返回拥有数据的 String，或者借用调用者传入的数据。

[返回目录](SUMMARY.md)
