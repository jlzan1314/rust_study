# unsafe：需要你自己证明安全的操作

unsafe 不会关闭所有检查。它允许执行少数编译器无法自动证明安全的操作，例如解引用裸指针、调用 unsafe 函数、访问可变静态变量、实现 unsafe trait、读取 union 字段。

```rust
let mut value = 5;
let pointer = &mut value as *mut i32;
// 指针来自仍存活的独占引用；此处没有其他并发或冲突访问。
unsafe {
    *pointer += 1;
}
assert_eq!(value, 6);
```

创建裸指针不一定需要 unsafe，解引用需要。裸指针可能为空、悬垂、未对齐，使用前必须由程序员保证有效。

标准库的 split_at_mut 是一个例子：它在内部检查边界并用底层操作构造互不重叠的可变切片，对外提供安全接口。不要只因“能编译”就认为手写版本正确。

与 C 交互通过 extern 声明接口，需要保证 ABI、指针、所有权和数据布局正确。Rust 2024 对 extern 块、部分属性和 unsafe 函数体的规则更明确，写法应参照当前工具链。

尽量缩小 unsafe 范围，并在旁边说明安全成立的具体理由。普通借用、类型检查仍然有效，unsafe 也不能让数据竞争或无效内存访问变得合法。

[返回目录](SUMMARY.md)
