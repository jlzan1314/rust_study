use clap::Args;

/// sum 子命令的参数。
#[derive(Debug, Args)]
pub struct SumArgs {
    #[arg(required = true, num_args = 1.., allow_negative_numbers = true)]
    numbers: Vec<i64>,
}

impl SumArgs {
    pub fn run(self) {
        // 用 i128 累加 i64 输入，给示例的求和运算留出余量。
        let total: i128 = self.numbers.into_iter().map(i128::from).sum();
        println!("{total}");
    }
}
