//! 默认入口只展示运行方式，各学习模块由 src/bin/ 下的独立入口执行。

fn main() {
    println!("请选择一个学习模块单独运行：\n");
    println!("所有权：       cargo run --bin ownership");
    println!("引用与借用：   cargo run --bin borrowing");
    println!("字符串切片：   cargo run --bin slice");
    println!("面向对象：     cargo run --bin object_oriented");
    println!("结构体：       cargo run --bin structs");
    println!("枚举与匹配：   cargo run --bin enums_and_matching");
    println!("动态数组 Vec： cargo run --bin vectors");
    println!("字符串 UTF-8： cargo run --bin strings");
    println!("哈希表：       cargo run --bin hash_maps");
    println!("错误处理：     cargo run --bin error_handling");
    println!("泛型与生命周期：cargo run --bin generics");
    println!("库 crate 调用：cargo run --bin math");
}
