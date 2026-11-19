# Rust 简明版目录

[Rust：用明白话学编程](title-page.md)
[前言：先写起来](foreword.md)
[这本书怎么读](ch00-00-introduction.md)

## 从这里开始

- [第 1 章：开始写 Rust](ch01-00-getting-started.md)
  - [安装 Rust](ch01-01-installation.md)
  - [运行第一个程序](ch01-02-hello-world.md)
  - [用 Cargo 管理项目](ch01-03-hello-cargo.md)

- [写一个猜数字游戏](ch02-00-guessing-game-tutorial.md)

- [第 3 章：变量、类型、函数和流程控制](ch03-00-common-programming-concepts.md)
  - [变量、可变性和遮蔽](ch03-01-variables-and-mutability.md)
  - [先分清数据类型](ch03-02-data-types.md)
  - [函数：输入、处理、返回值](ch03-03-how-functions-work.md)
  - [注释写“为什么”](ch03-04-comments.md)
  - [条件判断和循环](ch03-05-control-flow.md)

- [第 4 章：所有权和借用](ch04-00-understanding-ownership.md)
  - [所有权：谁负责释放数据](ch04-01-what-is-ownership.md)
  - [引用：使用数据，但不拿走它](ch04-02-references-and-borrowing.md)
  - [切片：借用连续的一段数据](ch04-03-slices.md)

- [第 5 章：用结构体组织数据](ch05-00-structs.md)
  - [结构体：把相关数据放在一起](ch05-01-defining-structs.md)
  - [用结构体表达一个矩形](ch05-02-example-structs.md)
  - [方法：把操作放到类型上](ch05-03-method-syntax.md)

- [第 6 章：用枚举表示不同情况](ch06-00-enums.md)
  - [枚举：一个值有几种可能](ch06-01-defining-an-enum.md)
  - [match：分别处理每种情况](ch06-02-match.md)
  - [只关心一种情况时，用 if let](ch06-03-if-let.md)

## 写项目常用的功能

- [第 7 章：把项目拆成模块](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
  - [Package 和 crate 不是一回事](ch07-01-packages-and-crates.md)
  - [模块和可见性](ch07-02-defining-modules-to-control-scope-and-privacy.md)
  - [路径：告诉 Rust 去哪里找名字](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
  - [use：给路径一个短名字](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
  - [把模块放进文件](ch07-05-separating-modules-into-different-files.md)

- [第 8 章：常用集合](ch08-00-common-collections.md)
  - [Vec：长度可以变化的数组](ch08-01-vectors.md)
  - [String 和 &str：拥有文本与借用文本](ch08-02-strings.md)
  - [HashMap：用键找值](ch08-03-hash-maps.md)

- [第 9 章：处理失败](ch09-00-error-handling.md)
  - [panic：程序无法继续时停止](ch09-01-unrecoverable-errors-with-panic.md)
  - [Result：把失败交给调用者处理](ch09-02-recoverable-errors-with-result.md)
  - [什么时候返回错误，什么时候 panic](ch09-03-to-panic-or-not-to-panic.md)

- [第 10 章：泛型、trait 和生命周期](ch10-00-generics.md)
  - [泛型：同一份逻辑处理多种类型](ch10-01-syntax.md)
  - [trait：约定一个类型能做什么](ch10-02-traits.md)
  - [生命周期：引用能用到什么时候](ch10-03-lifetime-syntax.md)

- [第 11 章：用测试验证行为](ch11-00-testing.md)
  - [写测试：给输入，检查结果](ch11-01-writing-tests.md)
  - [运行测试和筛选测试](ch11-02-running-tests.md)
  - [单元测试、集成测试和文档测试](ch11-03-test-organization.md)

- [第 12 章：写一个命令行搜索工具](ch12-00-an-io-project.md)
  - [读取命令行参数](ch12-01-accepting-command-line-arguments.md)
  - [读取文本文件](ch12-02-reading-a-file.md)
  - [把参数、业务逻辑和程序入口分开](ch12-03-improving-error-handling-and-modularity.md)
  - [先写清搜索规则，再写测试](ch12-04-testing-the-librarys-functionality.md)
  - [用环境变量控制搜索方式](ch12-05-working-with-environment-variables.md)
  - [正常结果和错误分开输出](ch12-06-writing-to-stderr-instead-of-stdout.md)

## 组织代码和共享数据

- [第 13 章：闭包和迭代器](ch13-00-functional-features.md)
  - [闭包：可以使用周围变量的小函数](ch13-01-closures.md)
  - [迭代器：逐项取值，串起处理步骤](ch13-02-iterators.md)
  - [用迭代器简化 minigrep](ch13-03-improving-our-io-project.md)
  - [循环和迭代器，哪个更快](ch13-04-performance.md)

- [第 14 章：Cargo 的常用进阶操作](ch14-00-more-about-cargo.md)
  - [开发版和发布版怎么选](ch14-01-release-profiles.md)
  - [发布库之前，先让别人能用](ch14-02-publishing-to-crates-io.md)
  - [工作空间：一起管理多个 Package](ch14-03-cargo-workspaces.md)
  - [cargo install：安装命令行工具](ch14-04-installing-binaries.md)
  - [Cargo 扩展命令](ch14-05-extending-cargo.md)

- [第 15 章：智能指针：怎么拥有和访问数据](ch15-00-smart-pointers.md)
  - [Box：一个所有者，管理堆上的数据](ch15-01-box.md)
  - [Deref：让包装类型能像引用一样使用](ch15-02-deref.md)
  - [Drop：值销毁时做清理](ch15-03-drop.md)
  - [Rc：让多个所有者共享一份数据](ch15-04-rc.md)
  - [RefCell：把借用检查放到运行时](ch15-05-interior-mutability.md)
  - [Weak：别让相互引用变成泄漏](ch15-06-reference-cycles.md)

- [第 16 章：多线程协作](ch16-00-concurrency.md)
  - [线程：把任务交给另一条执行路径](ch16-01-threads.md)
  - [通道：通过发送消息协作](ch16-02-message-passing.md)
  - [共享可变数据：Arc 加 Mutex](ch16-03-shared-state.md)
  - [Send 和 Sync：能不能跨线程](ch16-04-extensible-concurrency-sync-and-send.md)

- [第 17 章：用数据、行为和状态设计程序](ch17-00-oop.md)
  - [Rust 怎样组织数据和行为](ch17-01-what-is-oo.md)
  - [trait 对象：同一个列表，放不同类型](ch17-02-trait-objects.md)
  - [状态模式：把允许的操作写清楚](ch17-03-oo-design-patterns.md)

## 高级主题

- [第 18 章：模式匹配](ch18-00-patterns.md)
  - [模式不只出现在 match 里](ch18-01-all-the-places-for-patterns.md)
  - [模式是否可能匹配失败](ch18-02-refutability.md)
  - [常用模式写法](ch18-03-pattern-syntax.md)

- [第 19 章：需要时再学的高级功能](ch19-00-advanced-features.md)
  - [unsafe：需要你自己证明安全的操作](ch19-01-unsafe-rust.md)
  - [trait 的几种进阶用法](ch19-03-advanced-traits.md)
  - [类型别名、never 和动态大小](ch19-04-advanced-types.md)
  - [函数指针和返回闭包](ch19-05-advanced-functions-and-closures.md)
  - [宏：在编译时生成代码](ch19-06-macros.md)

- [第 20 章：写一个简单的多线程服务器](ch20-00-final-project-a-web-server.md)
  - [先写一个能返回 HTTP 响应的服务器](ch20-01-single-threaded.md)
  - [用固定线程池处理连接](ch20-02-multithreaded.md)
  - [关闭线程池：先停止派发，再等待完成](ch20-03-graceful-shutdown-and-cleanup.md)

- [附录：需要时查](appendix-00.md)
  - [关键字按用途记](appendix-01-keywords.md)
  - [常见运算符和符号](appendix-02-operators.md)
  - [derive：让编译器生成常见实现](appendix-03-derivable-traits.md)
  - [常用工具：格式化、检查、文档](appendix-04-useful-development-tools.md)
  - [edition：选择语言规则，不是锁定编译器](appendix-05-editions.md)
  - [译本和来源](appendix-06-translation.md)
  - [stable、beta 和 nightly 怎么选](appendix-07-nightly-rust.md)
