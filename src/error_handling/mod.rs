//! 第 9 章：错误处理。运行：cargo run --bin error_handling
//! 参考：https://rustwiki.org/zh-CN/book/ch09-00-error-handling.html
//!
//! 可预期、可恢复的错误返回 Result；无法继续执行的程序错误可用 panic!。
//! 会中断运行的示例保留为注释，正常运行可以观察完整流程。

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read, Write};
use std::path::Path;

/// 演示 panic、匹配错误、错误传播，以及用类型约束合法输入。
pub fn run() -> io::Result<()> {
    println!("\n===== 第 9 章：错误处理 =====");
    panic_examples();
    result_examples();
    file_examples()?;
    validated_type();
    Ok(())
}

/// panic 会终止当前线程；主线程 panic 通常意味着程序失败退出。
fn panic_examples() {
    println!("\n1. 不可恢复的错误：panic!");
    // 取消注释即可观察主动 panic；后面的代码将不会继续执行。
    // panic!("crash and burn");

    let numbers = vec![1, 2, 3];
    // 下标越界也会触发 panic，而不是读取任意内存。
    // println!("{}", numbers[99]);
    println!("使用 get 安全处理越界：{:?}", numbers.get(99));
    // 调试 panic 时可运行：RUST_BACKTRACE=1 cargo run --bin error_handling
}

/// Result<T, E> 有 Ok(T) 和 Err(E) 两种成员，无需自行定义或导入。
fn result_examples() {
    println!("\n2. 可恢复的错误：Result、unwrap 和 expect");
    for input in ["42", "不是数字"] {
        // parse 返回 Result<i32, ParseIntError>；类型参数指定解析目标。
        match input.parse::<i32>() {
            Ok(number) => println!("解析 {input:?} 成功：{number}"),
            Err(error) => println!("解析 {input:?} 失败：{error}"),
        }
    }

    // unwrap/expect 在 Ok 时取出值，在 Err 时 panic。
    // expect 可以补充说明为什么这里理应成功；用户输入通常应使用 match 或 ?。
    let home: std::net::IpAddr = "127.0.0.1".parse().unwrap();
    let port = "8080".parse::<u16>().expect("写死的端口应是合法的 u16");
    println!("已知合法的常量：{home}:{port}");
    // let invalid = "abc".parse::<i32>().expect("这里需要一个整数");
}

/// 使用独占创建的临时文件，避免在项目中创建或覆盖 hello.txt。
fn file_examples() -> io::Result<()> {
    println!("\n3. 文件错误与错误传播");
    // 使用 create_new 独占创建带进程号的临时文件；已存在则报错，避免覆盖。
    let path = std::env::temp_dir().join(format!("rust-study-errors-{}.txt", std::process::id()));
    let mut file = File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&path)?;
    // 创建成功后，无论后续操作返回 Ok 还是 Err，都尝试清理这个文件。
    let result = (|| -> io::Result<()> {
        file.write_all(b"Rust learner")?;
        drop(file);

        // 同一种读取操作，分别使用 match、? 和标准库便捷函数实现。
        println!("match 传播：{}", read_with_match(&path)?);
        println!("? 传播：{}", read_with_question_mark(&path)?);
        println!("便捷函数：{}", fs::read_to_string(&path)?);

        // 删除刚创建的文件，保证随后能观察到 NotFound，而不依赖外部文件。
        fs::remove_file(&path)?;
        match File::open(&path) {
            Ok(_) => println!("文件已存在"),
            Err(error) => match error.kind() {
                // 只有不存在才创建；其他错误（如无权限）交给调用方。
                ErrorKind::NotFound => {
                    let _file = File::options().write(true).create_new(true).open(&path)?;
                    println!("匹配到 NotFound，已重新创建文件");
                }
                _ => return Err(error),
            },
        }
        // 新文件为空，不是读取错误；读取成功返回 Ok("")。
        println!("重新创建后的内容：{:?}", read_with_question_mark(&path)?);
        Ok(())
    })();
    let cleanup = fs::remove_file(&path);
    result?;
    cleanup?;
    Ok(())
}

/// 手动传播：本函数决定不了如何处理错误，就把 Err 返回给调用方。
fn read_with_match(path: &Path) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(error), // 提前结束当前函数。
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text), // 成功时忽略读取的字节数，返回文本。
        Err(error) => Err(error),
    }
}

/// ? 在 Ok 时取出值，在 Err 时提前返回错误（必要时通过 From 转换错误类型）。
fn read_with_question_mark(path: &Path) -> io::Result<String> {
    // io::Result<String> 是 Result<String, io::Error> 的类型别名。
    let mut text = String::new();
    File::open(path)?.read_to_string(&mut text)?;
    // ? 不会自动包装成功结果，因此末尾仍要写 Ok(text)。
    // 返回 () 的普通函数不能直接对 Result 使用 ?；这里的返回类型支持传播。
    Ok(text)
}

/// 私有字段阻止外部直接构造任意数值，保证 Guess 始终在 1..=100 内。
pub struct Guess {
    value: i32,
}

impl Guess {
    /// 与书中一致：调用方违反构造约定时 panic。
    /// 若数值来自用户输入且不合法是正常情况，更适合返回 Result。
    pub fn new(value: i32) -> Self {
        if !(1..=100).contains(&value) {
            panic!("猜测值必须在 1 到 100 之间，实际为 {value}");
        }
        Self { value }
    }

    /// 提供只读访问，避免外部绕过检查修改字段。
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn validated_type() {
    println!("\n4. 用类型保证数值范围");
    let guess = Guess::new(42);
    println!("合法猜测：{}", guess.value());
    // let invalid = Guess::new(101); // 违反构造约定，会 panic。
}
