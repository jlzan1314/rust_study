//! 第 13 章：闭包、迭代器与函数式风格。
//! 参考：https://rustwiki.org/zh-CN/book/ch13-00-functional-features.html
//! 运行：cargo run --bin functional

pub mod closures;
pub mod iterators;
pub mod minigrep;

pub fn run() {
    closures::run();
    iterators::run();
    let contents = "Rust:\nsafe, fast, productive.\nTrust me.";
    println!("迭代器搜索：{:?}", minigrep::search("Rust", contents));
}
