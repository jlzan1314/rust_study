//! 库 crate 调用示例。运行：cargo run --bin math

// crate 名 → 模块名 → 函数名。use 让下面可以直接写 add。
// 当前二进制中的 crate:: 指向自己，访问库时需要使用库名 rust_study。
use rust_study::math::add;

fn main() {
    println!("库 crate 示例：2 + 3 = {}", add(2, 3));
}
