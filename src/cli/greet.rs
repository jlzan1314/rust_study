use clap::Args;

/// greet 子命令的参数。
#[derive(Debug, Args)]
pub struct GreetArgs {
    /// 必填的位置参数。
    name: String,

    /// 问候次数，取值范围为 1 到 10。
    #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=10))]
    count: u8,

    /// 使用大写输出。
    #[arg(short, long)]
    uppercase: bool,
}

impl GreetArgs {
    pub fn run(self) {
        let greeting = format!("Hello, {}!", self.name);
        let greeting = if self.uppercase {
            greeting.to_uppercase()
        } else {
            greeting
        };
        for _ in 0..self.count {
            println!("{greeting}");
        }
    }
}
