# Rust 简明版：本地中文书

从 [全书目录](SUMMARY.md) 开始阅读，也可以查看 [扉页与版本说明](title-page.md)、[前言](foreword.md) 和 [简介](ch00-00-introduction.md)。正文与插图保存在此目录，可直接用编辑器的 Markdown 预览阅读，无需构建 mdBook。

这是根据原书整理的简明改写版，不是逐字翻译。按“用途、规则、短例子、容易出错的地方”解释，删去重复铺垫、反复重构的中间版本和冗长输出。保留章节顺序与文件名，目录可直接跳转。

## 与项目学习示例对应

| 书中章节 | 本地书籍 | 项目示例与说明 |
|---|---|---|
| 第 7 章：包、crate 与模块 | [简明版](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md) | [项目结构与运行命令](../cate.md) |
| 第 8 章：常见集合 | [简明版](ch08-00-common-collections.md) | [集合与方法速查](../collections.md) |
| 第 9 章：错误处理 | [简明版](ch09-00-error-handling.md) | [错误处理示例](../error_handling.md) |
| 第 10 章：泛型、trait 与生命周期 | [简明版](ch10-00-generics.md) | [泛型、trait 与生命周期示例](../generics.md) |
| 第 11 章：自动化测试 | [简明版](ch11-00-testing.md) | [测试示例](../testing.md) |
| 第 12 章：I/O 项目 | [简明版](ch12-00-an-io-project.md) | [minigrep 示例](../io_project.md) |
| 第 13 章：闭包与迭代器 | [简明版](ch13-00-functional-features.md) | [闭包、迭代器与搜索示例](../functional.md) |
| 第 15 章：智能指针 | [简明版](ch15-00-smart-pointers.md) | [智能指针示例](../smart_pointers.md) |

其他章节可从全书目录进入。项目文档索引见 [docs/README.md](../README.md)。

## 来源与本地处理

- 在线阅读：[Rust Wiki 中文版](https://rustwiki.org/zh-CN/book/)。
- 中文源码：[rust-lang-cn/book-cn](https://github.com/rust-lang-cn/book-cn)。
- 固定版本：[`cde74c448e301ce8ac7960a0d3dc879efd83635d`](https://github.com/rust-lang-cn/book-cn/tree/cde74c448e301ce8ac7960a0d3dc879efd83635d)。
- 保存日期：2026-11-19；使用上述源码快照，在线站点的部署内容可能有所不同。
- 本地改写日期：2026-11-19。上述版本记录的是改写依据，不代表当前正文与上游完全一致。
- 保留目录层次、章节文件名、插图资源及许可证；正文、标题和代码示例按简明阅读需要重新整理。
- 章间链接使用本地 `.md` 路径；标准库等外部资料仍使用在线链接。
- 例子直接写在正文中，不依赖 mdBook 的 include 指令，也不再显示 rustdoc 隐藏辅助行。

本项目使用 Rust 2024 edition。可独立运行的 Rust 例子按该 edition 检查；需要第三方依赖、多文件上下文、启动网络服务或演示失败退出的代码会另行说明。需要完整展开的原书讲解时，使用上方在线原文或固定版本源码链接。

原作者为 Steve Klabnik、Carol Nichols 及 Rust 社区贡献者，中文翻译由 Rust 中文翻译项目组及社区维护。许可证见 [LICENSE-MIT](LICENSE-MIT) 与 [LICENSE-APACHE](LICENSE-APACHE)，保留原始版权及授权声明。
