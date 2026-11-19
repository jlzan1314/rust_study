# stable、beta 和 nightly 怎么选

Rust 工具链有三个常见发布通道：

| 通道 | 用途 |
|---|---|
| stable | 稳定版，普通学习和应用开发优先使用 |
| beta | 即将发布版本的测试通道 |
| nightly | 开发中的版本，可试用部分未稳定功能 |

```bash
rustup toolchain install nightly
cargo +nightly check
cargo +stable check
```

`+nightly` 只为这次命令选择工具链，不必修改默认版本。使用未稳定特性通常还要添加对应的 feature 属性，代码也可能随开发变化。

Rust 通常按约六周的节奏发布稳定版。想试验新特性可以用 nightly，但如果项目需要它，应记录理由和固定日期，方便别人复现。

[返回目录](SUMMARY.md)
