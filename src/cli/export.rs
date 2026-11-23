use clap::{ArgAction, Args};

use super::common::{OutputArgs, OutputFormat};

#[derive(Debug, Args)]
#[command(
    visible_alias = "ex",
    after_help = "示例：clap_demo export --tag rust --tag cli -ddd --format json"
)]
pub struct ExportArgs {
    #[command(flatten)]
    output: OutputArgs,

    /// 可多次传入，例如 --tag rust --tag cli。
    #[arg(long)]
    tag: Vec<String>,

    /// 重复计数：-d、-dd、-ddd。
    #[arg(short, long, action = ArgAction::Count)]
    debug: u8,
}

impl ExportArgs {
    pub fn run(self) {
        match self.output.format {
            OutputFormat::Text => {
                println!("标签：{}；调试等级：{}", self.tag.join(", "), self.debug)
            }
            // 仅序列化数量与等级，避免手工转义用户输入的字符串。
            OutputFormat::Json => println!(
                "{{\"tag_count\":{},\"debug\":{}}}",
                self.tag.len(),
                self.debug
            ),
        }
    }
}
