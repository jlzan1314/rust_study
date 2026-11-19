//! 13.3：对照第 12 章，将参数解析与搜索改为迭代器风格。

use crate::io_project::Config;
use std::env;

/// 接管参数迭代器，next 直接移出 String，避免第 12 章切片版本中的 clone。
/// impl Iterator<Item = String> 同时接受 env::args() 和 Vec<String>::into_iter()。
pub fn build_config(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next(); // 跳过程序名。
    let query = args.next().ok_or("缺少搜索字符串")?;
    let file_path = args.next().ok_or("缺少文件路径")?;
    if args.next().is_some() {
        return Err("参数过多：只接受搜索字符串和文件路径");
    }
    Ok(Config {
        query,
        file_path,
        ignore_case: env::var_os("IGNORE_CASE").is_some(),
    })
}

/// filter 的闭包捕获 query，collect 驱动迭代并收集原始行的切片。
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/// 复用第 12 章的配置类型，使用本章迭代器搜索实现。
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(&config.file_path)?;
    let matches = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in matches {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_search_matches_chapter_12() {
        for contents in ["", "Rust:\r\nTrust me.\r\n\r\n学习 Rust", "one\ntwo\n"] {
            for query in ["", "Rust", "rUsT", "学习", "missing"] {
                assert_eq!(
                    search(query, contents),
                    crate::io_project::search(query, contents)
                );
                assert_eq!(
                    search_case_insensitive(query, contents),
                    crate::io_project::search_case_insensitive(query, contents)
                );
            }
        }
    }

    #[test]
    fn parses_owned_arguments_and_rejects_wrong_count() {
        let config = build_config(
            ["minigrep", "Rust", "poem.txt"]
                .map(String::from)
                .into_iter(),
        )
        .unwrap();
        assert_eq!(config.query, "Rust");
        assert_eq!(config.file_path, "poem.txt");
        for args in [
            vec![],
            vec!["bin"],
            vec!["bin", "Rust"],
            vec!["bin", "Rust", "file", "extra"],
        ] {
            assert!(build_config(args.into_iter().map(String::from)).is_err());
        }
    }
}
