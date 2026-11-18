//! 10.2：trait 描述共享行为，trait bound 限定泛型类型需要具备的能力。

use std::fmt::Display;

trait Summary {
    fn summarize_author(&self) -> String;

    // 默认实现可以调用同一个 trait 中的其他方法，具体类型也可以覆盖它。
    fn summarize(&self) -> String {
        format!("阅读更多来自 {} 的内容", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    //这里直接覆盖了
    fn summarize(&self) -> String {
        format!("{}，作者：{}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // 不实现 summarize，直接使用 trait 的默认行为。
}

// impl Trait 参数接受任何实现 Summary 的类型；& 表示只借用，不获取所有权。
fn notify(item: &impl Summary) {
    println!("通知：{}", item.summarize());
}

// 与上面单参数的 impl Summary 写法等价，显式声明泛型和约束。
fn notify_generic<T: Summary>(item: &T) {
    println!("泛型通知：{}", item.summarize());
}

// where 将较长的约束移到参数列表之后；+ 表示同时满足多个 trait。
fn announce<T>(item: &T)
where
    T: Summary + Display,
{
    println!("展示：{item}；摘要：{}", item.summarize());
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "用户 @{}", self.username)
    }
}

// 返回 impl Trait 隐藏具体类型，但所有返回分支必须是同一个具体类型。
fn sample_summary() -> impl Summary {
    Tweet {
        username: String::from("rust_learner"),
    }
    // 不能在另一个分支中改为返回 NewsArticle，即使它也实现了 Summary。
}

struct Pair<T> {
    x: T,
    y: T,
}

// 条件实现：只有满足这些约束的 Pair<T> 才有 cmp_display 方法。
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("较大的值是 x：{}", self.x);
        } else {
            println!("较大的值是 y：{}", self.y);
        }
    }
}

pub fn run() {
    println!("\n2. trait：定义并约束共享行为");
    let article = NewsArticle {
        headline: String::from("开始学习 Rust 泛型"),
        author: String::from("小明"),
    };
    let tweet = Tweet {
        username: String::from("小红"),
    };
    notify(&article);
    notify_generic(&tweet);
    announce(&tweet);
    println!("返回 impl Summary：{}", sample_summary().summarize());
    Pair { x: 10, y: 20 }.cmp_display();
    // 标准库为实现 Display 的类型提供 ToString 的 blanket implementation。
    println!("Display 带来的 to_string：{}", tweet.to_string());
    // 孤儿规则：实现 trait 时，trait 或目标类型至少有一个必须属于当前 crate。
}
