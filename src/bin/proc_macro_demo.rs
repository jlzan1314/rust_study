//! 运行：cargo run --bin proc_macro_demo
//! 宏定义：study_macros/src/lib.rs

use study_macros::Describe;

// 原结构体会保留；Describe 读取注释，额外生成 impl Options。
#[derive(Debug, Describe)]
struct Options {
    /// 服务监听地址。
    host: String,
    /// 服务监听端口。
    port: u16,
}

fn main() {
    println!("1. 调用 derive 宏生成的方法：");
    Options::describe();

    println!("\n2. 原结构体仍然可以正常使用：");
    let options = Options {
        host: "localhost".to_string(),
        port: 8080,
    };
    println!("{options:?}");
    println!("地址：{}:{}", options.host, options.port);

    println!("\n试着修改字段的 /// 注释，再运行，生成的字段说明也会变化。");
    println!("注释在编译期由宏读取；运行时只执行生成的 println! 代码。");
}
