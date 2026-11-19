//! 第 13 章迭代器版本：cargo run --bin minigrep_iter -- to docs/poem.txt
use rust_study::functional::minigrep;
use std::{env, process};

fn main() {
    let config = minigrep::build_config(env::args()).unwrap_or_else(|err| {
        eprintln!("参数错误：{err}");
        process::exit(1);
    });
    if let Err(err) = minigrep::run(config) {
        eprintln!("程序错误：{err}");
        process::exit(1);
    }
}
