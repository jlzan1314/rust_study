//! 默认入口只展示运行方式，各学习模块由 src/bin/ 下的独立入口执行。

fn main() {
    println!("clap 参数解析：cargo run --bin clap_demo -- --help");
    println!("请选择一个学习模块单独运行：\n");
    println!("所有权：       cargo run --bin ownership");
    println!("引用与借用：   cargo run --bin borrowing");
    println!("字符串切片：   cargo run --bin slice");
    println!("面向对象：     cargo run --bin object_oriented");
    println!("结构体：       cargo run --bin structs");
    println!("枚举与匹配：   cargo run --bin enums_and_matching");
    println!("库 crate 调用：cargo run --bin math");
    println!("属性过程宏：   cargo run --bin attribute_macro");
}
