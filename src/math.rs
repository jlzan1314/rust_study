//! 库 crate 中的 math 模块；这是库的一个模块，不是单独的 crate。

/// 返回两个整数的和。
/// pub 允许库的使用者通过 rust_study::math::add 调用这个函数。
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}
