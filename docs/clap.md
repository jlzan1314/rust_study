# clap 命令行参数解析

示例源码：[clap_demo.rs](../src/bin/clap_demo.rs)。依赖启用 `derive` feature 后，可以通过结构体和枚举声明命令行接口。

示例按职责拆分；新增子命令时，在独立模块定义参数和实现，再注册到 `Commands`：

```text
src/
├── bin/clap_demo.rs   # 入口：解析参数、分发命令
└── cli/
    ├── mod.rs        # Parser 顶层参数、Subcommand 子命令枚举
    ├── greet.rs      # Args 问候参数与实现
    ├── sum.rs        # Args 求和参数与实现
    ├── common.rs     # ValueEnum、flatten 共享参数
    ├── export.rs     # 枚举、重复选项、计数、别名
    ├── connect.rs    # 环境变量、自定义校验、参数约束
    └── config.rs     # 嵌套子命令
```

`#[derive(Args)]` 定义一组参数，通过 `Greet(GreetArgs)` 等枚举变体接入子命令。

```toml
[dependencies]
clap = { version = "4", features = ["derive", "env"] }
```

## 运行示例

```bash
# 查看自动生成的帮助和版本
cargo run --bin clap_demo -- --help
cargo run --bin clap_demo -- --version
cargo run --bin clap_demo -- greet --help

# 位置参数 name；可选参数 count 默认是 1
cargo run --bin clap_demo -- greet Rust
cargo run --bin clap_demo -- greet Rust -c 2 --uppercase

# 全局布尔开关，详细模式提示写入标准错误
cargo run --bin clap_demo -- --verbose greet Rust

# Vec 接收多个整数，支持负数
cargo run --bin clap_demo -- sum 10 -3 5

# 参数校验失败：count 只接受 1 到 10
cargo run --bin clap_demo -- greet Rust --count 0
```

`cargo run` 后的 `--` 用于分隔 Cargo 自己的参数与传给示例程序的参数。

## 核心写法

| 写法 | 作用 |
|---|---|
| `#[derive(Parser)]` | 为顶层结构体生成参数解析实现，调用 `Cli::parse()` 解析 |
| `#[derive(Subcommand)]` | 将枚举变体映射为子命令，如 `Greet` 对应 `greet` |
| `#[command(version, about)]` | 从 Cargo 元数据读取版本，从文档注释读取说明 |
| `#[command(subcommand)]` | 将字段声明为子命令 |
| `#[arg(short, long)]` | 根据字段名生成短选项与长选项，如 `-c`、`--count` |
| `default_value_t = 1` | 不传选项时使用默认值 |
| `value_parser` | 解析并校验参数类型、范围 |
| `global = true` | 允许在子命令前后使用该选项 |

没有 `short` / `long` 的普通字段作为位置参数；`bool` 默认作为开关，`Vec<T>` 可以接收多个值。缺少必填参数、无法转换类型或超出范围时，clap 会打印错误并以非零状态退出。

## 更多功能用例

以下命令均在项目根目录执行。

```bash
# ValueEnum 限定输出格式；Vec 接收重复选项；Count 统计 -d 的次数
cargo run --bin clap_demo -- export --format json --tag rust --tag cli -ddd
# 输出：{"tag_count":2,"debug":3}

# visible_alias 为 export 提供 ex 别名；帮助中附带运行示例
cargo run --bin clap_demo -- ex --tag rust
cargo run --bin clap_demo -- export --help

# env：环境变量作为默认输入（需启用 env feature）
CLAP_DEMO_HOST=example.com CLAP_DEMO_PORT=9000 cargo run --bin clap_demo -- connect --anonymous

# 优先级：命令行 > 环境变量 > default_value
CLAP_DEMO_PORT=9000 cargo run --bin clap_demo -- connect --anonymous --port 7000

# Option<String>：用户和密码可省略；指定用户后必须同时提供密码
# 使用演示值即可；此命令不发起网络请求
cargo run --bin clap_demo -- connect --user alice --password example

# 嵌套子命令；show 和 export 通过 flatten 复用 OutputArgs
cargo run --bin clap_demo -- config show --format json
cargo run --bin clap_demo -- config set theme dark
# set 仅打印模拟结果，不写入配置文件
```

以下命令故意触发校验错误，程序退出码为 2：

```bash
# ValueEnum：不支持 xml
cargo run --bin clap_demo -- export --format xml

# 自定义 value_parser：端口不能为 0
cargo run --bin clap_demo -- connect --anonymous --port 0

# ArgGroup：必须在 --user 与 --anonymous 中选择一个
cargo run --bin clap_demo -- connect

# requires：指定用户却没有密码
cargo run --bin clap_demo -- connect --user alice

# ArgGroup 默认不允许多选
cargo run --bin clap_demo -- connect --anonymous --user alice --password example

# conflicts_with：密码与匿名模式冲突
cargo run --bin clap_demo -- connect --anonymous --password example
```

| 功能 | 定义位置 | 关键写法 |
|---|---|---|
| 枚举值和参数复用 | `common.rs` | `ValueEnum`、`#[command(flatten)]` |
| 重复参数和计数 | `export.rs` | `Vec<String>`、`ArgAction::Count` |
| 别名和帮助示例 | `export.rs` | `visible_alias`、`after_help` |
| 环境变量和可选参数 | `connect.rs` | `env`、`Option<String>` |
| 参数组、依赖、冲突 | `connect.rs` | `ArgGroup`、`requires`、`conflicts_with` |
| 自定义校验 | `connect.rs` | `value_parser = parse_port` |
| 嵌套子命令 | `config.rs` | `Args` 中再声明 `#[command(subcommand)]` |

入口继续使用 `Cli::parse()`，解析失败时由 clap 打印错误并退出。需要自行处理错误时，可以改用 `Cli::try_parse()`；[参数解析测试](../tests/clap_examples.rs) 使用 `try_parse_from` 验证合法输入与约束错误，无需退出测试进程。

## TOML 配置文件：config + serde

[serve.rs](../src/cli/serve.rs) 使用成熟的 `config` crate 合并配置源，使用 `serde` 反序列化成 `ServerSettings`，由 clap 解析命令行覆盖项。示例只打印最终配置，不启动服务。

```toml
# Cargo.toml 中的额外依赖：仅开启需要的 TOML 支持
config = { version = "0.15", default-features = false, features = ["toml"] }
serde = { version = "1", features = ["derive"] }
```

示例文件：[examples/server.toml](../examples/server.toml)。

```toml
host = "0.0.0.0"
port = 3000
workers = 8
```

按顺序试运行，观察端口变化：

```bash
# 代码默认值：127.0.0.1:8080，4 个工作线程
cargo run --bin clap_demo -- serve

# 文件：0.0.0.0:3000，8 个工作线程
cargo run --bin clap_demo -- serve --config examples/server.toml

# 环境变量覆盖文件：端口 9000
CLAP_SERVER_PORT=9000 cargo run --bin clap_demo -- serve --config examples/server.toml

# CLI 覆盖环境变量：端口 7000，其他字段沿用文件
CLAP_SERVER_PORT=9000 cargo run --bin clap_demo -- serve --config examples/server.toml --port 7000

# 所有字段都支持环境变量及 CLI 覆盖
CLAP_SERVER_HOST=localhost CLAP_SERVER_WORKERS=2 cargo run --bin clap_demo -- serve
cargo run --bin clap_demo -- serve --host localhost --port 5000 --workers 2

# 错误示例：配置校验失败、显式指定的文件不存在
CLAP_SERVER_PORT=0 cargo run --bin clap_demo -- serve
cargo run --bin clap_demo -- serve --config examples/not-present.toml
```

合并顺序是 **代码默认值 → TOML 文件 → 环境变量 → CLI 覆盖项**，后者优先。

CLI 覆盖字段使用 `Option<T>`，不设置 clap 的 `default_value`，这样没有传入的参数就不会覆盖文件中的值。环境变量由 `config::Environment` 统一读取，前缀为 `CLAP_SERVER_`。不传 `--config` 时不读取文件；显式传入的文件必须存在且是有效 TOML。

`try_deserialize()` 检查字段类型和数值范围，随后统一检查非空地址、非零端口和线程数，因此从文件或环境变量读取的值也经过校验。配置结构体使用 `deny_unknown_fields`，拼错字段名会报错。配置加载失败打印原因并以状态码 1 退出，CLI 参数错误由 clap 处理并以状态码 2 退出。

运行配置测试：

```bash
cargo test --test clap_config_examples
```
