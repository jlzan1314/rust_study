use clap::{Args, Subcommand};

use super::common::{OutputArgs, OutputFormat};

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommand,
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    /// 展示示例配置，复用输出格式参数。
    Show {
        #[command(flatten)]
        output: OutputArgs,
    },
    /// 演示 key/value 位置参数，不持久化。
    Set { key: String, value: String },
}

impl ConfigArgs {
    pub fn run(self) {
        match self.command {
            ConfigCommand::Show { output } => match output.format {
                OutputFormat::Text => println!("theme = light"),
                OutputFormat::Json => println!("{{\"theme\":\"light\"}}"),
            },
            ConfigCommand::Set { key, value } => println!("模拟设置：{key} = {value}（未保存）"),
        }
    }
}
