use clap::{ArgGroup, Args};

#[derive(Debug, Args)]
#[command(group(ArgGroup::new("auth").required(true).args(["user", "anonymous"])))]
pub struct ConnectArgs {
    /// 命令行优先，其次环境变量，最后使用默认值。
    #[arg(long, env = "CLAP_DEMO_HOST", default_value = "localhost")]
    host: String,

    /// 自定义解析器校验非零端口。
    #[arg(long, env = "CLAP_DEMO_PORT", default_value = "8080", value_parser = parse_port)]
    port: u16,

    /// 指定用户后必须同时提供 --password。
    #[arg(long, requires = "password")]
    user: Option<String>,

    /// 密码必须配合 --user；与匿名模式冲突。
    #[arg(long, requires = "user", conflicts_with = "anonymous")]
    password: Option<String>,

    /// 与 --user 二选一，由 auth 参数组约束。
    #[arg(long)]
    anonymous: bool,
}

fn parse_port(value: &str) -> Result<u16, String> {
    let port = value
        .parse::<u16>()
        .map_err(|_| "端口必须是 1 到 65535 的整数".to_string())?;
    if port == 0 {
        return Err("端口不能为 0".to_string());
    }
    Ok(port)
}

impl ConnectArgs {
    pub fn run(self) {
        // 只展示解析结果，不发起网络请求，也不输出密码。
        let identity = self.user.as_deref().unwrap_or("anonymous");
        println!("连接目标：{}:{}；身份：{identity}", self.host, self.port);
    }
}
