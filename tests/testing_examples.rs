//! 11.3：集成测试。这个文件会作为独立 crate 编译，只能使用库的公开 API。
//! 运行：cargo test --test testing_examples

use rust_study::testing::{Rectangle, add_two, greeting};

// 放在目录的 mod.rs 中，不会被 Cargo 当作独立的集成测试入口。
mod testing_common;

#[test]
fn public_functions_work_together() {
    let name = testing_common::sample_name();
    let message = greeting(name);
    let count = add_two(1);
    assert_eq!(
        format!("{message} 共 {count} 次练习"),
        "你好，集成测试！ 共 3 次练习"
    );
    // rust_study::testing::internal_adder(1, 2); // 编译失败：私有函数。
}

#[test]
fn public_rectangle_api_handles_equal_height() {
    let outer = Rectangle {
        width: 8,
        height: 7,
    };
    let inner = Rectangle {
        width: 5,
        height: 7,
    };
    assert!(!outer.can_hold(&inner));
}
