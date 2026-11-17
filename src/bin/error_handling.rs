//! 错误处理示例。运行：cargo run --bin error_handling

// main 返回 Result，可以使用 ?；Err 会让程序以失败状态退出并输出错误。
fn main() -> std::io::Result<()> {
    rust_study::error_handling::run()?;
    Ok(())
}
