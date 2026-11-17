//! 8.2：String 与 UTF-8。运行：cargo run --bin strings
//! 参考：https://rustwiki.org/zh-CN/book/ch08-02-strings.html
//!
//! String 拥有文本数据，&str 借用文本。字符串长度和切片范围以字节计算。

/// 演示字符串的创建、追加、拼接、UTF-8 切片和遍历。
pub fn run() {
    println!("\n===== 8.2：String 与 UTF-8 =====");
    create_strings();
    append_text();
    concatenate_strings();
    utf8_and_slices();
    iterate_text();
}

/// String::new 创建空字符串，from 和 to_string 根据已有文本创建 String。
fn create_strings() {
    println!("\n1. 创建 String 和借用为 &str");
    let empty = String::new();
    let from_literal = String::from("你好，Rust");
    let converted = "hello".to_string();
    println!("String::new()：{empty:?}");
    println!("String::from(...)：{from_literal}");
    println!("to_string()：{converted}");

    // as_str 返回指向现有文本的切片，不复制数据，也不转移所有权。
    let borrowed: &str = from_literal.as_str();
    println!("借用得到的 &str：{borrowed}");
    println!("原 String 仍然有效：{from_literal}");
}

/// push 接受一个 char，push_str 接受一段 &str。
fn append_text() {
    println!("\n2. 用 push 和 push_str 追加文本");
    let mut greeting = String::from("你好");
    let name = String::from("Rust");
    println!("追加前：{greeting}");

    greeting.push('，'); // 单引号表示一个 char，它不一定只占一个字节。
    // &String 可以自动转换为 &str；push_str 只借用 name，并复制其文本到末尾。
    greeting.push_str(&name);
    greeting.push('!');
    println!("追加后：{greeting}");
    println!("被借用的 name 仍可使用：{name}");
}

/// + 会消耗左侧 String 并借用右侧文本；format! 借用参数并返回新 String。
fn concatenate_strings() {
    println!("\n3. + 与 format! 的所有权区别");
    let left = String::from("hello, ");
    let right = String::from("Rust");
    let joined = left + &right;
    println!("+ 拼接结果：{joined}");
    println!("右侧字符串仍然有效：{right}");
    // println!("{left}"); // 编译错误 E0382：left 的所有权已经移动。

    let first = String::from("你好");
    let second = String::from("Rust");
    let formatted = format!("{first}，{second}！");
    println!("format! 拼接结果：{formatted}");
    println!("原来的两个 String 仍可使用：{first} / {second}");
    // format! 返回 String；println! 则把格式化内容输出到终端。
}

/// len 返回字节数；切片范围必须落在 UTF-8 字符边界上。
fn utf8_and_slices() {
    println!("\n4. UTF-8 字节长度与字符串切片");
    let text = "你好Rust";
    println!("输入：{text}");
    println!("len() 字节数：{}", text.len()); // 两个汉字各 3 字节，Rust 占 4 字节。
    println!("chars().count() 标量值数量：{}", text.chars().count());
    println!("底层字节：{:?}", text.as_bytes());

    let first = &text[0..3]; // [0, 3) 恰好包含完整的“你”，类型为 &str。
    println!("有效切片 [0..3]：{first}");
    println!("get(0..3)：{:?}", text.get(0..3));
    println!("get(0..1)：{:?}，不是有效字符边界", text.get(0..1));
    println!("get(99..100)：{:?}，超出字符串长度", text.get(99..100));

    // 错误示例 1：字符串不支持单个整数下标，下面会编译失败。
    // let character = text[0];
    // 错误示例 2：范围切片切断了汉字的 UTF-8 编码，下面会在运行时 panic。
    // let broken = &text[0..1];
    // 范围不确定时可使用 get，它对越界和非字符边界都返回 None。
}

/// chars 按 Unicode 标量值遍历，bytes 按 UTF-8 原始字节遍历。
fn iterate_text() {
    println!("\n5. 按 Unicode 标量值和字节遍历");
    let text = "你A";
    println!("输入：{text}");
    for character in text.chars() {
        println!("chars()：{character}");
    }

    print!("bytes()：");
    for byte in text.bytes() {
        print!("{byte} ");
    }
    println!();

    // char_indices 同时给出字节起始位置与 char，不是“第几个可见字符”。
    for (byte_index, character) in text.char_indices() {
        println!("字节位置 {byte_index}：{character}");
    }

    // e 加上组合重音符，显示时通常是一个字形，但实际上有两个 Unicode 标量值。
    let combined = "e\u{301}";
    println!(
        "组合文本 {combined}：{} 字节，{} 个 Unicode 标量值",
        combined.len(),
        combined.chars().count()
    );
    // chars().count() 不等于通用的“可见字符数”；字形簇分割需要额外的库支持。
}
