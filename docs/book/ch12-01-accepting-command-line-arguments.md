# 读取命令行参数

这一章写一个简化版 grep：接收关键词和文件路径，打印包含关键词的行。调用方式：

```bash
cargo run --bin minigrep -- Rust docs/poem.txt
```

Cargo 后面的 `--` 表示剩余参数交给我们的程序。用 `std::env::args()` 读取时，第一项通常是程序路径，要跳过。

```rust
fn parse(mut args: impl Iterator<Item = String>)
    -> Result<(String, String), &'static str>
{
    args.next();
    let query = args.next().ok_or("缺少关键词")?;
    let path = args.next().ok_or("缺少文件路径")?;
    if args.next().is_some() { return Err("参数过多"); }
    Ok((query, path))
}
let args = ["minigrep", "Rust", "poem.txt"].map(String::from);
assert_eq!(parse(args.into_iter()).unwrap().0, "Rust");
```

不要直接索引参数 Vec 并假设用户一定传对。先检查数量，再给清楚的提示。

args 要求参数是有效 Unicode；需要完整支持系统路径等非 Unicode 参数时，使用 args_os 和 PathBuf。

[返回目录](SUMMARY.md)
