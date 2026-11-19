# 把模块放进文件

在 lib.rs 中写：

```rust,ignore
pub mod tools;
```

这表示声明 tools 模块，编译器会到 `src/tools.rs` 或 `src/tools/mod.rs` 找内容。两种布局选一种，不要同时创建。

例如：

```text
src/
├── lib.rs             # pub mod tools;
└── tools/
    ├── mod.rs         # pub mod format;
    └── format.rs      # format 模块中的函数
```

`tools/mod.rs` 中的 `pub mod format;` 声明子模块；实际函数放在 format.rs。调用路径仍是 `crate::tools::format::函数名`，文件拆分不会改变模块含义。

创建文件不等于声明模块，通常还需要 mod。use 负责引入路径中的名字，不能代替模块声明。Cargo 对 src/bin 下二进制入口的自动发现属于另一套规则。

[返回目录](SUMMARY.md)
