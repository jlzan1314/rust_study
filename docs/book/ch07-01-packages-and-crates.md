# Package 和 crate 不是一回事

Package 是 Cargo 管理的项目，根目录有 Cargo.toml。crate 是编译单元，分为库和二进制两种。

| 项目中的位置 | 表示什么 |
|---|---|
| `src/lib.rs` | 一个库 crate 的入口 |
| `src/main.rs` | 一个二进制 crate 的入口 |
| `src/bin/tool.rs` | 另一个名为 tool 的二进制 crate |

一个 Package 最多有一个库 crate，可以有多个二进制 crate，至少有一个 crate。模块只是 crate 内部的组织方式，不是新的 crate。

库用来提供功能，二进制用来启动程序。当前项目的 `src/bin/smart_pointers.rs` 调用 `rust_study::smart_pointers::run()`，就是二进制使用同一个 Package 中的库。

详细结构见[项目说明](../cate.md)。

[返回目录](SUMMARY.md)
