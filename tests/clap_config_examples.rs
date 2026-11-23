use std::process::{Command, Output};

fn run(args: &[&str], port: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_clap_demo"));
    command.current_dir(env!("CARGO_MANIFEST_DIR"));
    // 子进程单独设置环境，避免并行测试修改进程级环境变量。
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("CLAP_SERVER_") {
            command.env_remove(key);
        }
    }
    if let Some(port) = port {
        command.env("CLAP_SERVER_PORT", port);
    }
    command.args(args).output().unwrap()
}

#[test]
fn configuration_priority_is_cli_then_env_then_file_then_defaults() {
    for (args, env_port, expected) in [
        (vec!["serve"], None, "port = 8080"),
        (
            vec!["serve", "--config", "examples/server.toml"],
            None,
            "port = 3000",
        ),
        (
            vec!["serve", "--config", "examples/server.toml"],
            Some("9000"),
            "port = 9000",
        ),
        (
            vec![
                "serve",
                "--config",
                "examples/server.toml",
                "--port",
                "7000",
            ],
            Some("9000"),
            "port = 7000",
        ),
    ] {
        let result = run(&args, env_port);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(String::from_utf8_lossy(&result.stdout).contains(expected));
    }
}

#[test]
fn unspecified_cli_fields_preserve_file_values() {
    let result = run(
        &[
            "serve",
            "--config",
            "examples/server.toml",
            "--port",
            "7000",
        ],
        None,
    );
    assert!(result.status.success());
    let output = String::from_utf8_lossy(&result.stdout);
    assert!(output.contains("host = 0.0.0.0"));
    assert!(output.contains("workers = 8"));
}

#[test]
fn invalid_environment_config_fails_without_panicking() {
    for port in ["0", "65536", "abc"] {
        let result = run(&["serve"], Some(port));
        assert_eq!(result.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&result.stderr).contains("配置错误"));
    }
}

#[test]
fn explicitly_requested_missing_file_fails() {
    let result = run(&["serve", "--config", "examples/not-present.toml"], None);
    assert_eq!(result.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&result.stderr).contains("配置错误"));
}
