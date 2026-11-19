# 用迭代器简化 minigrep

参数本来就是迭代器，可以逐项 next，不必先 collect 成 Vec 再复制元素。传入 `impl Iterator<Item = String>`，还方便测试时构造输入。

搜索也能直接表达为“按行取出，保留匹配的行，收集结果”：

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}
assert_eq!(search("Rust", "Rust\nGo"), vec!["Rust"]);
```

闭包借用了 query，返回的行借用 contents。改写后应保持原来的匹配规则、错误输出和退出码。

本项目保留两套实现，便于比较：

- [循环版本](../../src/io_project/mod.rs)。
- [迭代器版本](../../src/functional/minigrep.rs)。
- [对两套 CLI 运行相同输入的测试](../../tests/functional_examples.rs)。

重构的目标是让表达更清楚，改完后要检查行为是否保持一致。

[返回目录](SUMMARY.md)
