# Rust 学习文档

## 本地书籍

[本地中文版说明与示例索引](book/README.md) · [Rust 简明版全书目录](book/SUMMARY.md)

改写后的简明正文、目录和插图保存在 `docs/book`，各学习文档的参考章节链接已指向本地书籍。

## 项目说明与学习示例

| 文档 | 内容 |
|---|---|
| [项目结构](cate.md) | Package、crate、模块、运行命令 |
| [常见集合](collections.md) | Vec、String、HashMap、集合方法与迭代器速查 |
| [错误处理](error_handling.md) | panic、Result、错误传播 |
| [泛型、trait 与生命周期](generics.md) | 泛型约束、共享行为、引用有效性 |
| [自动化测试](testing.md) | 单元测试、集成测试与文档测试 |
| [I/O 命令行项目](io_project.md) | minigrep 参数、文件读取、搜索与错误输出 |
| [闭包与迭代器](functional.md) | 闭包捕获、迭代器、搜索重构 |
| [智能指针](smart_pointers.md) | Box、Deref、Drop、Rc、RefCell、Weak |
| [clap 参数解析](clap.md) | derive、位置参数、选项、子命令与参数校验 |
| [JSON 处理](json.md) | serde_json、结构体转换、动态字段和错误处理 |
| [类型推断](type_inference.md) | 上下文、集合、泛型、闭包、Option/Result 与 `_` |
| [Tokio 异步示例](../src/bin/tokio_demo.rs) | async/await、join!、spawn、mpsc 与超时 |
| [eBPF 命令监控](ebpf_monitor.md) | Aya 原生实现，按 UID、命令名统计执行次数、开始时间与耗时 |
| [Axum 路由测试](../tests/axum_examples.rs) | oneshot 请求、JSON 响应断言、404 和 405；运行 `cargo test --test axum_examples` |
