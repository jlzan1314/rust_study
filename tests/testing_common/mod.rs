//! 集成测试共享的辅助模块，每次返回固定数据，不依赖运行顺序。

pub fn sample_name() -> &'static str {
    "集成测试"
}
