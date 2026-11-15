//! 第 6 章「枚举和模式匹配」的源码示例。
//! 参考：https://rustwiki.org/zh-CN/book/ch06-00-enums.html
//!
//! 阅读顺序：定义枚举 → match → if let → 实际场景。
//! main.rs 调用本模块的 run()，执行 cargo run 即可查看输入和输出。

// Debug 让 println!("{value:?}") 可以打印枚举成员及其关联数据。
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,            // 1 美分，不携带数据。
    Nickel,           // 5 美分，不携带数据。
    Dime,             // 10 美分，不携带数据。
    Quarter(UsState), // 25 美分，同时保存一个州名。
}

/// 本章入口：按顺序运行所有示例，每个示例都有中文说明和输出。
pub fn run() {
    println!("\n===== 第 6 章：枚举和模式匹配 =====");
    defining_enums::run();
    match_patterns::run();
    if_let_patterns::run();
    usage_examples::run();
}

// 6.1：同一个枚举的成员属于同一类型，但可以携带不同的数据。
mod defining_enums {
    pub(super) fn run() {
        enum_variants();
        associated_data();
        enum_methods();
    }

    /// 6.1.1：V4 和 V6 都是 IpAddrKind 类型，可以传给同一个函数。
    fn enum_variants() {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }

        // &'static str 可以引用字符串字面量，其数据在整个程序运行期间有效。
        fn protocol(kind: &IpAddrKind) -> &'static str {
            // => 左边是模式，右边是匹配成功时求值的表达式。
            // match 末尾没有分号，匹配结果就是函数的返回值。
            match kind {
                IpAddrKind::V4 => "IPv4",
                IpAddrKind::V6 => "IPv6",
            }
        }

        println!("\n6.1.1：不同枚举成员可以传给同一个函数");
        // :: 访问枚举成员；&kind 借用枚举值，调用后仍然可以打印它。
        for kind in [IpAddrKind::V4, IpAddrKind::V6] {
            println!("输入：{kind:?} → 协议：{}", protocol(&kind));
        }
    }

    /// 6.1.2：不同成员可以携带不同类型、不同数量的数据。
    fn associated_data() {
        #[derive(Debug)]
        enum IpAddr {
            V4(u8, u8, u8, u8), // 四个 0..=255 的整数。
            V6(String),         // 一个拥有所有权的字符串。
        }

        fn address_text(address: IpAddr) -> String {
            match address {
                // 按位置解构，将四段地址分别绑定到 a、b、c、d。
                IpAddr::V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
                // 按值接收 address，内部 String 被移动到 text，再作为结果返回。
                IpAddr::V6(text) => text,
            }
        }

        println!("\n6.1.2：枚举成员携带数据");
        let addresses = [IpAddr::V4(192, 168, 1, 10), IpAddr::V6(String::from("::1"))];
        for address in addresses {
            println!("输入：{address:?}");
            println!("地址：{}", address_text(address));
        }
    }

    /// 6.1.3：枚举可以通过 impl 定义方法，也能在方法内解构成员。
    fn enum_methods() {
        #[derive(Debug)]
        enum Message {
            Quit,                    // 类单元成员：没有关联数据。
            Move { x: i32, y: i32 }, // 结构体式成员：按字段名存储数据。
            Write(String),           // 元组式成员：按位置存储数据。
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn describe(&self) -> String {
                // 匹配 &self 只借用数据，不会取走 Write 内部 String 的所有权。
                match self {
                    Message::Quit => String::from("退出"),
                    Message::Move { x, y } => format!("移动到 ({x}, {y})"),
                    Message::Write(text) => format!("文字：{text}"),
                    Message::ChangeColor(r, g, b) => format!("颜色 ({r}, {g}, {b})"),
                }
            }
        }

        println!("\n6.1.3：枚举的方法与解构");
        let messages = [
            Message::Quit,
            Message::Move { x: 3, y: -2 },
            Message::Write(String::from("你好")),
            Message::ChangeColor(255, 128, 0),
        ];
        for message in messages {
            println!("输入：{message:?} → {}", message.describe());
            println!("再次调用：{}", message.describe()); // 借用结束后仍可继续使用。
        }
    }
}

// 6.2：match 必须覆盖所有可能的情况，并且只执行第一个匹配的分支。
mod match_patterns {
    // super 表示父模块，这里复用本文件顶部定义的类型。
    use super::{Coin, UsState};

    pub(super) fn run() {
        coin_values();
        bind_variant_data();
        match_option();
        catch_all_pattern();
        ignore_remaining_values();
        exhaustive_match();
    }

    /// 6.2.1：match 是表达式，可以根据成员返回对应的值。
    fn coin_values() {
        fn value_in_cents(coin: &Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(_) => 25, // 只匹配 Quarter，_ 忽略其内部州名。
            }
        }

        println!("\n6.2.1：匹配硬币，返回面值");
        let coins = [
            Coin::Penny,
            Coin::Nickel,
            Coin::Dime,
            Coin::Quarter(UsState::Alabama),
            Coin::Quarter(UsState::Alaska),
        ];
        for coin in coins {
            println!("输入：{coin:?} → {} 美分", value_in_cents(&coin));
        }
    }

    /// 6.2.2：将枚举成员内保存的数据绑定到变量，供分支使用。
    fn bind_variant_data() {
        println!("\n6.2.2：提取硬币内部的州名");
        for coin in [Coin::Quarter(UsState::Alaska), Coin::Penny] {
            println!("输入：{coin:?}");
            let state = match coin {
                Coin::Quarter(state) => Some(state), // 提取州名，包装为 Some。
                _ => None, // 此处的 _ 匹配所有其他成员，这些硬币没有州名。
            };
            println!("提取结果：{state:?}");
        }
    }

    /// 6.2.3：Option<T> 用 Some(T) 表示有值，用 None 表示没有值。
    fn match_option() {
        fn plus_one(value: Option<i32>) -> Option<i32> {
            match value {
                Some(number) => Some(number + 1), // 提取数字，加一后重新包装。
                None => None,                     // 没有数字时保留 None。
            }
        }

        println!("\n6.2.3：匹配 Option，给存在的数字加一");
        for value in [Some(5), Some(-1), None] {
            println!("输入：{value:?} → 结果：{:?}", plus_one(value));
        }
        // Some(-1) 加一得到 Some(0)，它仍然是“有值”，与 None 不同。
    }

    /// 6.2.4：变量模式能匹配剩余任意值，并允许分支使用这个值。
    fn catch_all_pattern() {
        println!("\n6.2.4：用 other 接住其他点数");
        for roll in [3u8, 7, 9, 1] {
            let action = match roll {
                3 => String::from("获得帽子"),
                7 => String::from("失去帽子"),
                // other 是新绑定的变量，不是关键字，也不是与已有变量比较。
                // 它匹配任意值，放在最后才能让前面的特殊分支有机会执行。
                other => format!("前进 {other} 格"),
            };
            println!("点数：{roll} → {action}");
        }
    }

    /// 6.2.5：_ 不绑定剩余值；() 是单元值，可用于表示无需操作。
    fn ignore_remaining_values() {
        println!("\n6.2.5：用 _ 忽略其他点数");
        let mut actions = Vec::new();
        // 3u8 指定数字类型，也让同一数组的其他数字被推断为 u8。
        for roll in [3u8, 9, 7, 1] {
            println!("当前点数：{roll}");
            match roll {
                3 => actions.push("获得帽子"),
                7 => actions.push("失去帽子"),
                _ => (), // 9 和 1 不会追加任何操作。
            }
        }
        println!("记录的操作：{actions:?}");
    }

    /// 6.2.6：穷尽性在编译时检查，不能遗漏 Option 的 None 分支。
    fn exhaustive_match() {
        // 下面是故意写错的示例，取消注释后编译器会报 E0004：
        // fn invalid_plus_one(value: Option<i32>) -> Option<i32> {
        //     match value {
        //         Some(number) => Some(number + 1),
        //     }
        // }
        // 修复方法：在 match 中补上 None => None，参见上面的 match_option。
        println!("\n6.2.6：match 必须穷尽所有情况");
        println!("只处理 Some、遗漏 None 会在编译时触发 E0004，错误写法见源码注释。");
    }
}

// 6.3：只关注一个模式时可以用 if let，其余情况可忽略或交给 else。
mod if_let_patterns {
    use super::{Coin, UsState};

    pub(super) fn run() {
        match_specific_value();
        extract_some_value();
        handle_other_variants();
    }

    /// 6.3.1：Some(3) 是固定模式，只匹配数字恰好为 3 的情况。
    fn match_specific_value() {
        println!("\n6.3.1：if let 只关注 Some(3)");
        let mut matched = Vec::new();
        for value in [Some(0u8), Some(3), None, Some(8), Some(3)] {
            println!("输入：{value:?}");
            if let Some(3) = value {
                println!("匹配成功，记录 {value:?}");
                matched.push(value);
            }
        }
        println!("匹配结果：{matched:?}");
    }

    /// 6.3.2：Some(number) 匹配任意 Some，将内部数据绑定到 number。
    fn extract_some_value() {
        println!("\n6.3.2：if let 提取任意 Some 中的数字");
        let mut numbers = Vec::new();
        for value in [Some(0), None, Some(12)] {
            println!("输入：{value:?}");
            // = 左边是模式，右边是待匹配的值；不是普通赋值或 == 比较。
            if let Some(number) = value {
                numbers.push(number);
            }
        }
        println!("提取结果：{numbers:?}"); // 0 也会被提取，只有 None 被跳过。
    }

    /// 6.3.3：匹配成功提取州名，其他成员由 else 统一计数。
    fn handle_other_variants() {
        println!("\n6.3.3：if let 配合 else 处理其他硬币");
        let coins = [
            Coin::Penny,
            Coin::Quarter(UsState::Alaska),
            Coin::Nickel,
            Coin::Dime,
            Coin::Quarter(UsState::Alabama),
        ];
        let mut states = Vec::new();
        let mut other_coin_count = 0;
        for coin in coins {
            println!("输入：{coin:?}");
            if let Coin::Quarter(state) = coin {
                // state 只在匹配成功的分支中可用。
                states.push(state);
            } else {
                // 相当于 match 的兜底分支，处理 Penny、Nickel 和 Dime。
                other_coin_count += 1;
            }
        }
        println!("州名：{states:?}，其他硬币数量：{other_coin_count}");
    }
}

// 实际场景：将上面的语法应用到订单和用户名处理。
mod usage_examples {
    #[derive(Debug)]
    enum OrderStatus {
        PendingPayment,               // 待付款，不需要附加数据。
        Shipped(String),              // 已发货，保存物流单号。
        Cancelled { reason: String }, // 已取消，保存具名字段 reason。
    }

    pub(super) fn run() {
        order_status();
        optional_username();
        shipped_orders();
    }

    /// 场景 1：根据订单状态，向用户显示下一步提示。
    fn order_status() {
        println!("\n场景 1：match 根据订单状态决定提示");
        let orders = [
            OrderStatus::PendingPayment,
            OrderStatus::Shipped(String::from("SF1001")),
            OrderStatus::Cancelled {
                reason: String::from("用户主动取消"),
            },
        ];
        for status in orders {
            // 借用状态进行匹配，匹配后还能打印完整的订单状态。
            let hint = match &status {
                OrderStatus::PendingPayment => String::from("请先付款"),
                OrderStatus::Shipped(tracking_number) => format!("物流单号：{tracking_number}"),
                OrderStatus::Cancelled { reason } => format!("订单已取消：{reason}"),
            };
            println!("输入：{status:?} → {hint}");
        }
    }

    /// 场景 2：区分用户名存在、为空字符串和缺失这三种情况。
    fn optional_username() {
        println!("\n场景 2：match 处理可选用户名");
        for username in [Some("小明"), Some(""), None] {
            let greeting = match username {
                // 固定值模式放在前面，Some("") 表示有字符串，但内容为空。
                Some("") => String::from("你好，未命名用户！"),
                // Some(name) 匹配任意 Some，必须放在空字符串的特殊分支之后。
                Some(name) => format!("你好，{name}！"),
                None => String::from("你好，游客！"), // 根本没有用户名。
            };
            println!("输入：{username:?} → {greeting}");
        }
    }

    /// 场景 3：只提取已发货订单的物流单号，其他状态统一跳过并计数。
    fn shipped_orders() {
        println!("\n场景 3：if let 只关注已发货订单");
        let orders = [
            OrderStatus::PendingPayment,
            OrderStatus::Shipped(String::from("SF1001")),
            OrderStatus::Cancelled {
                reason: String::from("库存不足"),
            },
            OrderStatus::Shipped(String::from("YT2002")),
        ];
        let mut tracking_numbers = Vec::new();
        let mut skipped = 0;
        for order in &orders {
            // 借用订单和内部 String；物流单号切片引用的 orders 必须保持有效。
            if let OrderStatus::Shipped(number) = order {
                println!("输入：{order:?} → 提取物流单号 {number}");
                tracking_numbers.push(number.as_str());
            } else {
                println!("输入：{order:?} → 跳过");
                skipped += 1;
            }
        }
        println!("物流单号：{tracking_numbers:?}，跳过 {skipped} 个订单");
    }
}
