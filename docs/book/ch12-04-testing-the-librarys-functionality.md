# 先写清搜索规则，再写测试

搜索函数只处理文本，不读文件、不打印，测试更简单。下面按大小写敏感、子串匹配返回整行。

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}
let text = "Rust:\nsafe, fast, productive.\nTrust me.";
assert_eq!(search("duct", text), vec!["safe, fast, productive."]);
assert!(search("missing", text).is_empty());
```

返回的每一行借用 contents，因此结果不能比原始文本活得更久。query 只用于判断，不需要与返回值绑定同一个生命周期。

测试至少考虑：有匹配、无匹配、空输入、空关键词和中文。空关键词会匹配 lines 产生的每一行，这个规则应明确，而不是凭感觉猜。

开发时可以先写期望结果，让测试失败，再实现搜索直到通过。不要为了“按测试驱动流程走”增加没有意义的测试。

[返回目录](SUMMARY.md)
