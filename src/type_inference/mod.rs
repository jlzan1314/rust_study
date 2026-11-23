//! 类型推断在编译期完成，不是运行时改变类型，也不是隐式类型转换。

use std::{any::type_name, collections::HashMap, num::ParseIntError};

fn show<T: std::fmt::Debug>(label: &str, value: &T) {
    // type_name 只用于演示；其输出格式不应作为程序逻辑的依据。
    println!("{label}: {value:?}，类型 = {}", type_name::<T>());
}

fn identity<T>(value: T) -> T {
    value
}

// 普通函数的参数、返回值类型必须声明；函数体的期望类型也参与推断。
fn make_numbers() -> Vec<u16> {
    vec![1, 2, 3]
}

pub fn run() {
    literals_and_context();
    collections();
    generics_and_placeholders();
    closures();
    option_and_result();
    println!("\n推断不足的反例与修复方式见 docs/type_inference.md。");
}

fn literals_and_context() {
    println!("1. 字面量、表达式和上下文");
    // 没有其他约束时，整数字面量默认 i32，浮点字面量默认 f64。
    let integer = 42;
    let float = 1.5;
    let enabled = true;
    let text = "Rust";
    show("整数默认类型", &integer);
    show("浮点默认类型", &float);
    show("布尔值", &enabled);
    show("字符串字面量", &text);

    // 不是先创建 i32 再转换为 u16：字面量一开始就被推断为 u16。
    let port: u16 = 8080;
    let next_port = port + 1;//推断为 u16
    show("运算上下文", &next_port);
    let status = if enabled { 200_u16 } else { 500 };
    show("分支类型统一", &status);
    show("函数返回值上下文", &make_numbers());
}

fn collections() {
    println!("\n2. 从后续使用推断集合类型");
    let mut names = Vec::new();
    // 编译器结合整个函数中的约束推断，不要求声明时就出现元素类型。
    names.push(String::from("Rust"));
    show("push 确定 Vec 元素类型", &names);

    let mut scores = HashMap::new();
    scores.insert("Alice", 90_u8);
    show("insert 确定键和值类型", &scores);


    //从右边推断
    let values = [1_u16, 2, 3];
    show("数组元素类型和长度", &values);
    let pair = ("Rust", 2024_u16);
    show("元组各元素的类型", &pair);
}

fn generics_and_placeholders() {
    println!("\n3. 泛型参数：从入参或期望返回类型推断");
    let number = identity(7_u8); // 从入参得到 T = u8。
    show("泛型函数入参", &number);

    // parse 类似 config.try_deserialize：左侧决定泛型目标类型。
    let port: u16 = "8080".parse().expect("示例端口合法");
    let explicit_port = "8080".parse::<u16>().expect("示例端口合法");
    assert_eq!(port, explicit_port);
    show("左侧类型推断 parse 目标", &port);
    show("turbofish 显式指定泛型", &explicit_port);

    // collect 可以构造多种集合，所以声明 Vec；元素类型交给迭代器推断。
    let doubled: Vec<_> = (1_u16..=3).map(|n| n * 2).collect();
    show("Vec<_> 部分推断", &doubled);
    let total: u16 = doubled.iter().copied().sum();
    show("sum 的返回类型", &total);

    let parsed: Result<Vec<u16>, _> = ["10", "20"].into_iter().map(str::parse).collect();
    show("嵌套泛型中推断错误类型", &parsed);
}

fn closures() {
    println!("\n4. 闭包参数和返回类型");
    let length = |text: &str| text.len();
    show("根据闭包函数体推断返回类型", &length("Rust"));

    let double = |n| n * 2;
    // 这次调用将闭包参数约束为 u16；该闭包不能再用 String 等类型调用。
    let result = double(5_u16);
    show("根据调用推断闭包参数", &result);

    let lengths: Vec<_> = ["Rust", "serde"].iter().map(|word| word.len()).collect();
    show("迭代器约束闭包参数", &lengths);
}

fn option_and_result() {
    println!("\n5. Option / Result 的泛型约束");
    let mut selected = None;
    selected.get_or_insert(8080_u16);
    show("后续使用推断 Option<T>", &selected);

    let missing: Option<String> = None;
    show("None 需要上下文确定 T", &missing);

    // Ok 只能确定成功类型，错误类型需要其他上下文。
    let success: Result<_, ParseIntError> = Ok(42_u16);
    show("Result 的成功类型和错误类型", &success);
}
