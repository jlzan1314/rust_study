//! clap 解析 CLI，config 合并配置源，serde 将结果转换为强类型结构体。

use std::path::PathBuf;

use clap::Args;
use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Args)]
pub struct ServeArgs {
    /// TOML 配置文件；不传则使用默认值、环境变量及 CLI。
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,

    // 使用 Option 而不是 default_value，防止 CLI 默认值覆盖文件中的配置。
    /// 覆盖监听地址。
    #[arg(long)]
    host: Option<String>,

    /// 覆盖监听端口。
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..))]
    port: Option<u16>,

    /// 覆盖工作线程数。
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..))]
    workers: Option<u16>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ServerSettings {
    host: String,
    port: u16,
    workers: u16,
}

impl ServeArgs {
    fn load(&self) -> Result<ServerSettings, ConfigError> {
        let mut builder = Config::builder()
            .set_default("host", "127.0.0.1")?
            .set_default("port", 8080)?
            .set_default("workers", 4)?;

        if let Some(path) = &self.config {
            // 显式指定的文件必须存在；格式固定为 TOML，不依赖文件扩展名。
            builder = builder.add_source(File::from(path.as_path()).format(FileFormat::Toml));
        }

        // 后加入的配置源覆盖前面的源，例如 CLAP_SERVER_PORT=9000。
        builder = builder.add_source(
            Environment::with_prefix("CLAP_SERVER")
                .prefix_separator("_")
                .try_parsing(true),
        );

        // CLI 的优先级最高，只有用户实际传入时才覆盖。
        if let Some(host) = &self.host {
            builder = builder.set_override("host", host.clone())?;
        }
        if let Some(port) = self.port {
            builder = builder.set_override("port", port)?;
        }
        if let Some(workers) = self.workers {
            builder = builder.set_override("workers", workers)?;
        }

        let settings: ServerSettings = builder.build()?.try_deserialize()?;
        // 文件和环境变量也要校验，不能仅依赖 clap 的 CLI 校验。
        if settings.host.trim().is_empty() || settings.port == 0 || settings.workers == 0 {
            return Err(ConfigError::Message(
                "host 不能为空，port 和 workers 必须大于 0".into(),
            ));
        }
        Ok(settings)
    }

    pub fn run(self) -> Result<(), ConfigError> {
        let settings = self.load()?;
        println!("最终配置（仅演示，不启动服务）：");
        println!("host = {}", settings.host);
        println!("port = {}", settings.port);
        println!("workers = {}", settings.workers);
        Ok(())
    }
}
