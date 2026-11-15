// mod borrowing;
// mod ownership;
// mod slice;
// mod object_oriented;
// mod structs;
mod enums_and_matching;

// 使用库 crate 的名称访问函数：crate 名 → 模块名 → 函数名。
// use 让下面可以直接写 add，而不必每次写 rust_study::math::add。
use rust_study::math::add;

fn main() {
    // Cargo 会先构建同一 Package 中的库，再让二进制 crate 调用它。
    // main.rs 中的 crate:: 指向二进制自身；访问库需要使用库名 rust_study。
    println!("库 crate 示例：2 + 3 = {}", add(2, 3));

    // ownership::run();
    // borrowing::run();
    // slice::run();
    // object_oriented::run();
    // structs::run();
    enums_and_matching::run();
}
