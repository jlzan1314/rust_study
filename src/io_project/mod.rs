//! 第 12 章：一个 I/O 项目——构建简化版 grep。
//! 参考：https://rustwiki.org/zh-CN/book/ch12-00-an-io-project.html
//! 运行：cargo run --bin minigrep -- to docs/poem.txt

use std::{env, error::Error, fs};

/// 将命令行配置集中到一个类型中，避免在各个函数之间传递零散参数。
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// 解析参数；args[0] 是程序名，真正的用户参数从下标 1 开始。
    ///
    /// 使用 Result 表示可预期的输入错误，让入口决定如何报告并退出。
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() != 3 {
            return Err("用法：minigrep <搜索字符串> <文件路径>");
        }

        Ok(Self {
            // args 是借用，clone 让配置拥有字符串，可以独立于参数集合存活。
            query: args[1].clone(),
            file_path: args[2].clone(),
            // 设置 IGNORE_CASE 即启用，包括 IGNORE_CASE=0 或空值。
            // var_os 只检查变量是否存在，也能处理非 Unicode 的变量值。
            ignore_case: env::var_os("IGNORE_CASE").is_some(),
        })
    }
}

/// 读取文件并输出匹配行；入口只处理参数与错误，业务逻辑留在库中。
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read_to_string 返回 UTF-8 String；路径不存在、权限不足、非 UTF-8 等均会报错。
    // ? 遇到 Err 会提前返回；Box<dyn Error> 可以容纳不同的错误类型。
    let contents = fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        // 成功结果写入 stdout，便于使用 > 重定向或通过管道交给其他程序。
        println!("{line}");
    }
    Ok(())
}

/// 区分大小写，返回包含 query 的原始行（不含换行符）。
///
/// 返回值借用 contents，因此生命周期 'a 与 contents 绑定，而不是 query。
/// 搜索的是字面子串，不支持正则表达式。
/// ```
/// use rust_study::io_project::search;
/// assert_eq!(search("duct", "Rust:\nsafe, fast, productive.\n"),
///            vec!["safe, fast, productive."]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // lines 同时支持 LF 和 CRLF；contains 进行区分大小写的子串匹配。
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

/// 将查询与行转为小写后比较，但返回原始文本，保留其大小写。
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        // to_lowercase 会分配新 String；这里的简化比较不等同于完整 Unicode case folding。
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    // 12.4 的 TDD 流程：先写预期行为并观察失败，再实现函数让测试通过，最后重构。
    #[test]
    fn case_sensitive() {
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(search("duct", contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive_preserves_original_lines() {
        let contents = "Rust:\nsafe, fast, productive.\nTrust me.";
        assert_eq!(
            search_case_insensitive("rUsT", contents),
            vec!["Rust:", "Trust me."]
        );
    }

    #[test]
    fn handles_no_matches_empty_input_and_empty_query() {
        assert!(search("missing", "Rust").is_empty());
        assert!(search("", "").is_empty());
        // 空字符串是任意字符串的子串，因此空查询匹配每一行。
        assert_eq!(search("", "one\r\n\r\ntwo\r\n"), vec!["one", "", "two"]);
    }

    #[test]
    fn searches_chinese_text() {
        assert_eq!(
            search("学习", "学习 Rust\n你好\n继续学习"),
            vec!["学习 Rust", "继续学习"]
        );
    }

    #[test]
    fn config_requires_query_and_path() {
        for args in [
            vec![],
            vec!["minigrep"],
            vec!["minigrep", "Rust"],
            vec!["minigrep", "Rust", "file", "extra"],
        ] {
            let args: Vec<String> = args.into_iter().map(String::from).collect();
            assert!(Config::build(&args).is_err());
        }
        let args = vec!["minigrep".into(), "Rust".into(), "poem.txt".into()];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "Rust");
        assert_eq!(config.file_path, "poem.txt");
        // 不修改进程环境：Rust 2024 中 set_var/remove_var 是 unsafe，且并行测试会互相影响。
        // 环境变量的行为由独立子进程集成测试验证。
    }
}
