//! 第 15 章：智能指针。运行：cargo run --bin smart_pointers
//! 参考：https://rustwiki.org/zh-CN/book/ch15-00-smart-pointers.html

pub mod boxes;
pub mod cycles;
pub mod deref;
pub mod drop_trait;
pub mod interior_mutability;
pub mod reference_counting;

pub fn run() {
    boxes::run();
    deref::run();
    drop_trait::run();
    reference_counting::run();
    interior_mutability::run();
    cycles::run();
}
