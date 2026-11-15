use std::process::Command;

/// 编译指定的错误示例，确认它编译失败且包含预期错误码。
/// fixture 是 tests/ui 下的文件名，expected_error 例如 "E0004" 或 "E0597"。
/// 测试通过意味着“编译器正确拒绝了错误代码”，不是示例编译成功。
pub fn assert_compile_fails(fixture: &str, expected_error: &str) {
    // env! 在编译测试时取得 Cargo.toml 所在目录，避免依赖运行时的工作目录。
    let project_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    // 进程号和示例名共同区分临时目录，避免不同测试的编译产物互相覆盖。
    let output_dir = std::env::temp_dir().join(format!(
        "rust-study-{}-{}",
        std::process::id(),
        fixture.trim_end_matches(".rs")
    ));
    std::fs::create_dir(&output_dir).expect("创建测试临时目录失败");

    // 错误代码单独交给 rustc 检查；放在 tests/ui 下不会被 Cargo 直接当作测试入口。
    // --emit=metadata 仍会检查类型、借用和模式，但无需生成可执行文件。
    let output = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
        .arg("--edition=2024")
        .arg("--emit=metadata")
        .arg("--out-dir")
        .arg(&output_dir)
        .arg(project_dir.join("tests/ui").join(fixture))
        .output();

    // 先清理临时文件，再判断编译结果；编译器的报错信息来自标准错误输出。
    std::fs::remove_dir_all(&output_dir).expect("清理测试临时目录失败");
    let output = output.expect("运行 Rust 编译器失败");
    let stderr = String::from_utf8_lossy(&output.stderr);

    // 同时检查“失败”和“错误码”，避免路径错误等无关故障被误判为测试通过。
    assert!(!output.status.success(), "{fixture} 不应该编译成功");
    assert!(
        stderr.contains(&format!("error[{expected_error}]")),
        "预期错误 {expected_error}，实际编译输出：\n{stderr}"
    );
}
