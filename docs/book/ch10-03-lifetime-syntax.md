# 生命周期：引用能用到什么时候

生命周期用于说明引用之间的有效期关系。它不会让数据多活一会儿，也不会改变实际释放时间。

```rust
fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
let a = String::from("hello");
let b = String::from("Rust");
assert_eq!(longer(&a, &b), "hello");
```

这里的 `'a` 表示：返回的引用可能来自任一输入，所以调用者只能在两份输入都有效的共同范围内使用它。比较按字节长度进行。

不能返回局部 String 的引用：

```rust,compile_fail
fn invalid<'a>() -> &'a str {
    let text = String::from("临时文本");
    &text // 函数结束后 text 就释放了。
}
```

## 什么时候需要手写

编译器能按省略规则推断时，不用写。例如 `fn first(text: &str) -> &str` 只有一个输入引用，输出生命周期默认跟它对应。方法有 `&self` 时，省略的返回引用通常与 self 对应。

结构体保存引用时，需要标明关系，例如 `struct Part<'a> { text: &'a str }`。Part 不能比被引用文本活得更久。

`&'static str` 表示引用的数据对整个程序都有效，字符串字面量符合这一点。不要为了消除报错就给普通局部引用加 static；标注不能把局部数据变成长寿命数据。

[返回目录](SUMMARY.md)
