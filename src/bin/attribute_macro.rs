//! 自定义属性宏示例。运行命令为 `cargo run --bin attribute_macro`。

use rust_study::{aop, log_call, math, struct_only};

#[struct_only]
struct Operands {
    left: i32,
    right: i32,
}

#[log_call]
fn add(left: i32, right: i32) -> i32 {
    math::add(left, right)
}

#[log_call("显示计算结果")]
fn show_result(result: i32) {
    println!("2 + 3 = {result}");
}

#[aop]
fn divide(dividend: i32, divisor: i32) -> Result<i32, &'static str> {
    if divisor == 0 {
        return Err("除数不能为零");
    }

    dividend.checked_div(divisor).ok_or("除法溢出")
}

fn main() {
    let operands = Operands { left: 2, right: 3 };
    show_result(add(operands.left, operands.right));

    assert_eq!(divide(10, 2), Ok(5));
    assert_eq!(divide(10, 0), Err("除数不能为零"));
}
