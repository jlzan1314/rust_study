//! 在子进程里控制环境，验证真实 CLI 的输出、退出码和环境变量。
use std::process::{Command, Output};

fn minigrep(args: &[&str], ignore_case: bool) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_minigrep"));
    command.current_dir(env!("CARGO_MANIFEST_DIR"));
    command.args(args).env_remove("IGNORE_CASE");
    if ignore_case {
        command.env("IGNORE_CASE", "1");
    }
    command.output().expect("无法运行 minigrep")
}

#[test]
fn searches_file_with_and_without_environment_variable() {
    let output = minigrep(&["to", "docs/poem.txt"], false);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Are you nobody, too?\nHow dreary to be somebody!\n"
    );
    assert!(output.stderr.is_empty());

    let output = minigrep(&["to", "docs/poem.txt"], true);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Are you nobody, too?\nHow dreary to be somebody!\nTo tell your name the livelong day\nTo an admiring bog!\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn reports_argument_and_file_errors_only_to_stderr() {
    for args in [vec![], vec!["Rust", "docs/poem.txt/not-a-file"]] {
        let output = minigrep(&args, false);
        assert_eq!(output.status.code(), Some(1));
        assert!(output.stdout.is_empty());
        assert!(!output.stderr.is_empty());
    }
}

#[test]
fn no_matches_is_success_with_empty_output() {
    let output = minigrep(&["不存在的字符串", "docs/poem.txt"], false);
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
}
