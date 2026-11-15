//! 库 crate 的根模块：提供可以被其他 crate 使用的功能，不需要 main 函数。
//!
//! Cargo 默认将 src/lib.rs 作为库入口，库名取自 Cargo.toml 的包名 rust_study。
//! src/main.rs 是同一个 Package 中另一个独立的二进制 crate。
//! 它通过 rust_study::math::add 调用这里公开的功能。

// 声明公开模块，从 src/math.rs 加载代码。
// 模块和函数都公开，其他 crate 才能通过完整路径访问 add。
pub mod math;
