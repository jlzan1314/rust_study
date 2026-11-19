//! 对两个版本运行相同 CLI 输入，验证迭代器重构保留外部行为。
use std::process::Command;

#[test]
fn iterator_cli_matches_original_outputs_and_exit_status() {
    for args in [
        vec!["to", "docs/poem.txt"],
        vec!["不存在的词", "docs/poem.txt"],
        vec![],
        vec!["Rust", "docs/poem.txt/not-a-file"],
        vec!["to", "docs/poem.txt", "extra"],
    ] {
        for ignore_case in [false, true] {
            let run = |binary| {
                let mut command = Command::new(binary);
                command
                    .current_dir(env!("CARGO_MANIFEST_DIR"))
                    .args(&args)
                    .env_remove("IGNORE_CASE");
                if ignore_case {
                    command.env("IGNORE_CASE", "1");
                }
                command.output().expect("无法执行 minigrep")
            };
            let original = run(env!("CARGO_BIN_EXE_minigrep"));
            let iterator = run(env!("CARGO_BIN_EXE_minigrep_iter"));
            assert_eq!(iterator.status.code(), original.status.code());
            assert_eq!(iterator.stdout, original.stdout);
            // 参数错误的具体描述可以不同，但必须输出到 stderr。
            assert_eq!(iterator.stderr.is_empty(), original.stderr.is_empty());
        }
    }
}
