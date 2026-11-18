//! 10.3：生命周期标注描述引用之间的有效期关系，不会延长数据的生命。

use std::fmt::Display;

// 返回值可能来自 x 或 y，显式用 'a 表达它们与返回值的借用关系。
// 调用时可以将两个引用缩短到共同有效的范围，并非要求原始数据同时销毁。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 结构体保存引用时，需要标注其依赖的生命周期；它不拥有被引用的文本。
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 生命周期省略规则：有 &self 时，省略的返回引用生命周期关联到 self。
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("提醒：{announcement}");
        self.part
    }
}

// 只有一个输入引用时，返回引用的生命周期可自动关联到这个输入。
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

// 同时使用生命周期参数、泛型参数和 trait bound。
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("公告：{announcement}");
    longest(x, y)
}

pub fn run() {
    println!("\n3. 生命周期：确保引用有效");
    let outer = String::from("long string is long");
    {
        let inner = String::from("xyz");
        let result = longest(&outer, &inner);
        println!("较长的文本：{result}"); // 在两个输入都有效时使用返回引用。
    }

    // 错误示例：返回引用不能逃离 inner 的有效范围，即使这次实际返回 outer。
    // let result;
    // {
    //     let inner = String::from("xyz");
    //     result = longest(&outer, &inner);
    // }
    // println!("{result}"); // E0597：inner 已被释放。

    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = ImportantExcerpt {
        part: novel.split('.').next().expect("固定的非空文本应有第一段"),
    };
    println!(
        "摘录：{}",
        excerpt.announce_and_return_part("原始文本仍然有效")
    );
    println!("省略生命周期：{}", first_word("hello Rust"));

    // 字符串字面量存放在程序中，引用可以在整个程序运行期间有效。
    let permanent: &'static str = "我是字符串字面量";
    println!("static 引用：{permanent}");
    // 'static 不是修复悬垂引用的万能标注，不能让局部 String 永久存活。
    println!(
        "综合示例：{}",
        longest_with_announcement(&outer, "Rust", "比较文本")
    );
    // 与书中一致，longest 按 len() 比较 UTF-8 字节数，并非可见字符数量。
}
