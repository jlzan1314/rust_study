use clap::{Args, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

/// 通过 flatten 在多个子命令之间复用。
#[derive(Debug, Args)]
pub struct OutputArgs {
    /// 输出格式；clap 自动生成可选值提示。
    #[arg(long, value_enum, default_value = "text")]
    pub format: OutputFormat,
}
