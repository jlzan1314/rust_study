//! 顶层参数与子命令注册。

mod common;
mod config;
mod connect;
mod export;
mod greet;
mod serve;
mod sum;

use clap::{Parser, Subcommand};
use config::ConfigArgs;
use connect::ConnectArgs;
use export::ExportArgs;
use greet::GreetArgs;
use serve::ServeArgs;
use sum::SumArgs;

/// 学习 clap 的参数、选项与子命令。
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// 开启详细输出；global 允许在子命令前后使用该选项。
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 向指定的人打招呼。
    Greet(GreetArgs),
    /// 对一个或多个整数求和，也支持负数。
    Sum(SumArgs),
    /// 枚举格式、重复选项与参数复用。
    Export(ExportArgs),
    /// 环境变量、参数依赖、冲突与分组。
    Connect(ConnectArgs),
    /// 嵌套子命令（仅演示，不读写配置文件）。
    Config(ConfigArgs),
    /// 合并 TOML、环境变量和命令行配置（仅展示，不启动服务）。
    Serve(ServeArgs),
}
