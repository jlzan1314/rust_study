# I/O 项目：构建命令行程序

参考 [Rust Book 第 12 章](book/ch12-00-an-io-project.md)，实现可以运行的简化版 `minigrep`。在项目根目录执行以下命令。

| 位置 | 内容 |
|---|---|
| [src/io_project/mod.rs](../src/io_project/mod.rs) | 配置解析、文件读取、搜索函数、中文注释与单元测试 |
| [src/bin/minigrep.rs](../src/bin/minigrep.rs) | 命令行入口、标准错误与退出状态 |
| [poem.txt](poem.txt) | 书中使用的 Emily Dickinson 诗歌样本 |
| [tests/io_project_examples.rs](../tests/io_project_examples.rs) | 真实子进程的端到端测试 |

## 12.1 接受命令行参数

```bash
cargo run --bin minigrep -- to docs/poem.txt
```

`--` 将 Cargo 的参数与程序参数分开。`env::args().collect()` 收集到 `Vec<String>`，第 0 项是程序名，第 1、2 项是搜索字符串与路径。含空格的参数需要引号。这里要求恰好两个用户参数，多余或缺少参数都会报错。

默认区分大小写，输出：

```text
Are you nobody, too?
How dreary to be somebody!
```

## 12.2 读取文件

`fs::read_to_string` 一次读取整个 UTF-8 文件。它返回 `Result<String, io::Error>`；文件不存在、权限不足、内容不是 UTF-8 等错误由 `?` 传递给调用者。文件路径相对于运行时的当前工作目录。

## 12.3 重构与错误处理

`Config::build(&args)` 负责验证参数并生成拥有字符串的配置，`run(config)` 负责读取与搜索，`main` 负责收集参数、显示错误、设置退出码。二进制通过 `rust_study::io_project` 调用库，不重复声明模块。

参数不足时，直接索引 `args[1]` 会 panic，因此先验证数量。配置错误返回 `Result<Config, &'static str>`；业务执行返回 `Result<(), Box<dyn Error>>`，借助 trait 对象统一传播错误。`unwrap_or_else` 中的闭包只在配置解析失败时执行。

## 12.4 测试驱动开发与生命周期

学习时可以先写 `case_sensitive` 测试，再暂时将 `search` 改为空结果以观察失败，然后恢复实现并重构。提交的示例保持测试通过。

`search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>` 返回文件内容中的切片，不复制匹配行。`'a` 说明结果不能比 `contents` 活得更久，与查询字符串的生命周期无关。`lines()` 拆行，`contains()` 判断字面子串，循环将匹配行放入 `Vec`。

```bash
cargo test --lib io_project::tests
cargo test --test io_project_examples
cargo test --doc io_project
```

测试覆盖大小写、中文、无匹配、空内容、空查询、CRLF、参数错误、读取错误与输出流。空查询匹配每一行；没有匹配时成功退出，输出为空。

## 12.5 环境变量

```bash
IGNORE_CASE=1 cargo run --bin minigrep -- to docs/poem.txt
```

这会额外匹配 `To` 开头的两行。只对当前命令设置环境变量，不影响后续命令。变量只要存在即启用，设置为 `0` 或空字符串也会启用；取消时可使用 `env -u IGNORE_CASE cargo run --bin minigrep -- to docs/poem.txt`。

搜索时将查询与每行转为小写，返回的仍是原始行。此方式沿用本章的简化实现，不支持正则表达式，也不实现完整 Unicode 大小写折叠。

项目使用 Rust 2024；测试通过 `Command::env` / `env_remove` 为子进程设置环境，避免使用不安全的进程级环境修改，也避免并行测试互相干扰。

## 12.6 标准输出与标准错误

```bash
# 搜索结果进入文件；错误仍显示在终端
cargo run --quiet --bin minigrep -- to docs/poem.txt > output.txt

# 参数错误：stdout 文件为空，错误进入 errors.txt，程序退出码为 1
cargo run --quiet --bin minigrep > output.txt 2> errors.txt
```

`println!` 写入 stdout，`eprintln!` 写入 stderr。运行成功（包括无匹配）退出码为 0，参数或文件错误为 1。上面的重定向命令会创建或覆盖输出文件。
