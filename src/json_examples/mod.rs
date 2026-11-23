//! serde 定义数据映射规则，serde_json 负责 JSON 的解析和生成。

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,
    // Rust 使用 user_name，JSON 中对应 userName。
    pub user_name: String,
    // 缺少 email 或值为 null 时都是 None；序列化时省略 None 字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    // JSON 未提供此字段时使用 Vec::default()，即空数组。
    #[serde(default)]
    pub skills: Vec<String>,
}

pub fn run(input: &str) -> Result<(), serde_json::Error> {
    println!("1. JSON 字符串 → 强类型结构体");
    let mut user: User = serde_json::from_str(input)?;
    println!("{user:?}");

    println!("\n2. 修改结构体 → JSON 字符串");
    user.skills.push("serde".into());
    println!("紧凑格式：{}", serde_json::to_string(&user)?);
    println!("格式化输出：\n{}", serde_json::to_string_pretty(&user)?);

    println!("\n3. 不定义结构体，用 Value 动态访问 JSON");
    let mut value: Value = serde_json::from_str(input)?;
    // get + as_str 返回 Option，可区分字段缺失或类型不符合预期。
    match value.get("userName").and_then(Value::as_str) {
        Some(name) => println!("userName = {name}"),
        None => println!("userName 缺失或不是字符串"),
    }
    // pointer 使用 JSON Pointer 定位数组中的元素。
    println!(
        "第一项技能：{:?}",
        value.pointer("/skills/0").and_then(Value::as_str)
    );
    value["active"] = json!(true);
    println!("新增字段：{}", serde_json::to_string(&value)?);

    println!("\n4. json! 构造 JSON，to_value / from_value 转换结构体");
    let envelope = json!({
        "success": true,
        "data": serde_json::to_value(&user)?,
    });
    println!("{}", serde_json::to_string_pretty(&envelope)?);
    let restored: User = serde_json::from_value(envelope["data"].clone())?;
    println!("还原用户：{}", restored.user_name);

    println!("\n5. 可选字段与默认值");
    let minimal: User = serde_json::from_str(r#"{"id":2,"userName":"小红"}"#)?;
    println!("email = {:?}, skills = {:?}", minimal.email, minimal.skills);
    println!("序列化省略 email：{}", serde_json::to_string(&minimal)?);

    println!("\n6. 捕获错误：语法错误与字段类型错误");
    for invalid in [r#"{"id": }"#, r#"{"id":"不是整数","userName":"小明"}"#] {
        match serde_json::from_str::<User>(invalid) {
            Ok(user) => println!("解析成功：{user:?}"),
            Err(error) => println!(
                "类别：{:?}；位置：第 {} 行、第 {} 列；原因：{error}",
                error.classify(),
                error.line(),
                error.column()
            ),
        }
    }
    Ok(())
}
