# JSON 处理

使用成熟的 `serde` + `serde_json`，入口是 [json_demo.rs](../src/bin/json_demo.rs)，主要实现在 [json_examples/mod.rs](../src/json_examples/mod.rs)。

```bash
# 使用编译时嵌入的示例，无需提供文件
cargo run --bin json_demo

# 从文件读取 UTF-8 JSON
cargo run --bin json_demo -- examples/user.json
```

示例依次演示：字符串与结构体转换、紧凑和美化输出、`Value` 动态访问与修改、`json!` 构造对象、结构体与 `Value` 互转、可选字段、默认值及错误处理。

| API / 属性 | 用途 |
|---|---|
| `#[derive(Serialize, Deserialize)]` | 生成序列化和反序列化实现 |
| `from_str::<User>()` | 将 JSON 字符串转换为强类型结构体 |
| `to_string()` / `to_string_pretty()` | 生成紧凑 / 美化的 JSON 字符串 |
| `Value`、`get()`、`as_str()` | 结构不固定时动态读取字段 |
| `pointer("/skills/0")` | 定位嵌套字段或数组元素 |
| `json!({...})` | 构造 JSON 值，字符串自动转义 |
| `to_value()` / `from_value()` | 在结构体与 JSON 值之间转换 |
| `rename_all = "camelCase"` | Rust 的 `user_name` 对应 JSON 的 `userName` |
| `Option<T>` | 处理缺失或为 null 的可选字段 |
| `#[serde(default)]` | 字段缺失时使用 Rust 默认值 |
| `skip_serializing_if = "Option::is_none"` | 输出时省略值为 None 的字段 |

字段类型不正确、必填字段缺失或 JSON 语法错误都会返回 `Result::Err`。示例捕获内置的错误用例后继续运行；用户传入的文件读取或解析失败时，入口打印错误并返回非零退出码。

本例默认忽略输入中的未知字段；需要严格拒绝未知字段时，可以在 `User` 上添加 `#[serde(deny_unknown_fields)]`。JSON 数组也可直接解析为 `Vec<User>`。
