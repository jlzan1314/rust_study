# 用环境变量控制搜索方式

环境变量适合传递不想每次都写在命令行上的配置。例如设置 IGNORE_CASE 开启忽略大小写搜索。

```bash
IGNORE_CASE=1 cargo run --bin minigrep -- rust docs/poem.txt
```

PowerShell 可以先执行 `$env:IGNORE_CASE="1"`，再运行程序。用完删除环境变量，避免影响后续命令。

如果使用 `std::env::var("IGNORE_CASE").is_ok()`，判断的是是否存在且能解码为 Unicode，不是值是否等于 1；设为 0 仍会开启。希望接受 true/false 时，需要自己解析。

```rust
fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
assert_eq!(search_insensitive("RUST", "Rust\nGo"), vec!["Rust"]);
```

只在比较时转小写，返回原始行。简单的 to_lowercase 比较不等同于完整的 Unicode 大小写折叠；面向多语言的严格搜索需要明确额外规则。

[返回目录](SUMMARY.md)
