//! 第 10 章：泛型、trait 和生命周期。
//! 参考：https://rustwiki.org/zh-CN/book/ch10-00-generics.html
//! 运行：cargo run --bin generics

pub mod lifetimes;
pub mod traits;
pub mod types;

/// 每节独立组织，运行时依次打印示例结果。
pub fn run() {
    println!("\n===== 第 10 章：泛型、trait 和生命周期 =====");
    types::run();
    traits::run();
    lifetimes::run();
}
