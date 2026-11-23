//! clap derive 示例：cargo run --bin clap_demo -- --help

use clap::Parser;
use rust_study::cli::{Cli, Commands};

fn main() {
    // 自动生成 --help / --version；参数错误时打印提示并以非零状态退出。
    let cli = Cli::parse();

    if cli.verbose {
        // 不打印整个 Cli，避免 connect 的密码进入日志。
        eprintln!("已开启详细输出");
    }

    match cli.command {
        Commands::Greet(args) => args.run(),
        Commands::Sum(args) => args.run(),
        Commands::Export(args) => args.run(),
        Commands::Connect(args) => args.run(),
        Commands::Config(args) => args.run(),
    }
}
