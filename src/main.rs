//! 默认入口只展示运行方式，各学习模块由 src/bin/ 下的独立入口执行。

fn main() {
    println!("过程宏演示：   cargo run --bin proc_macro_demo");
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
    println!("编写自动化测试：cargo test --lib testing::tests");
    println!("I/O 命令行项目：cargo run --bin minigrep -- to docs/poem.txt");
    println!("闭包与迭代器： cargo run --bin functional");
    println!("智能指针：     cargo run --bin smart_pointers");
    println!("迭代器版搜索： cargo run --bin minigrep_iter -- to docs/poem.txt");
    println!("库 crate 调用：cargo run --bin math");
    println!("clap 参数解析：cargo run --bin clap_demo -- --help");
    println!("JSON 处理：    cargo run --bin json_demo");
    println!("类型推断：     cargo run --bin type_inference");
    println!("Tokio 异步：   cargo run --bin tokio_demo");
}
