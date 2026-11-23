//! cargo run --bin json_demo -- [JSON 文件路径]

use std::{fs, path::PathBuf, process::ExitCode};

use clap::Parser;

/// serde_json 使用示例：默认读取内置示例，也可以传入 JSON 文件。
#[derive(Parser)]
struct Args {
    /// User 格式的 JSON 文件路径。
    file: Option<PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let input = match args.file {
        Some(path) => match fs::read_to_string(&path) {
            Ok(input) => input,
            Err(error) => {
                eprintln!("读取 {} 失败：{error}", path.display());
                return ExitCode::FAILURE;
            }
        },
        None => include_str!("../../examples/user.json").to_string(),
    };
    match rust_study::json_examples::run(&input) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("JSON 处理失败：{error}");
            ExitCode::FAILURE
        }
    }
}
