//! 命令行入口：cargo run --bin minigrep -- to docs/poem.txt

use rust_study::io_project::{self, Config};
use std::{env, process};

fn main() {
    // collect 将参数迭代器收集到 Vec；本章保留切片解析方式，便于学习索引与借用。
    // env::args 要求参数是 Unicode；第 13 章可进一步改为直接消费迭代器。
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        // 闭包只在 Err 时运行；错误写入 stderr，避免混入搜索结果。
        eprintln!("参数错误：{err}");
        process::exit(1);
    });

    if let Err(err) = io_project::run(config) {
        eprintln!("程序错误：{err}");
        process::exit(1);
    }
}
