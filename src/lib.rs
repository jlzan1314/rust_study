//! 库 crate 的根模块：提供可以被其他 crate 使用的功能，不需要 main 函数。
//!
//! Cargo 默认将 src/lib.rs 作为库入口，库名取自 Cargo.toml 的包名 rust_study。
//! src/main.rs 和 src/bin/ 下的各个入口分别属于独立的二进制 crate。
//! 各个入口通过 rust_study::模块名 调用对应功能，例如 rust_study::math::add。

// 统一声明学习模块；二进制只负责调用自己的模块，不重复定义模块。
// 模块和函数都公开，其他 crate 才能通过完整路径调用。
pub mod borrowing;
pub mod collections;
pub mod enums_and_matching;
pub mod math;
pub mod object_oriented;
pub mod ownership;
pub mod slice;
pub mod structs;
