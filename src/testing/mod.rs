//! 第 11 章：编写、运行和组织测试。
//! 参考：https://rustwiki.org/zh-CN/book/ch11-00-testing.html
//! 运行本模块单元测试：cargo test --lib testing::tests

/// 为输入加二，供单元测试、集成测试和文档测试调用。
///
/// 三个反引号中的 Rust 示例会被 cargo test 作为文档测试执行。
/// ```
/// use rust_study::testing::add_two;
/// assert_eq!(add_two(2), 4);
/// ```
pub fn add_two(value: i32) -> i32 {
    internal_adder(value, 2)
}

// 私有函数不能被外部 crate 调用，但本模块的测试子模块可以访问。
fn internal_adder(left: i32, right: i32) -> i32 {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("你好，{name}！")
}

/// 只判断方向不变时能否严格容纳；相同尺寸不算容纳，不尝试旋转。
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// cfg(test) 使这个模块仅在编译测试时参与构建，正常 cargo build 不包含它。
#[cfg(test)]
mod tests {
    // super 是父模块，导入本模块中需要验证的函数和类型。
    use super::*;
    use crate::error_handling::Guess;

    // #[test] 标记交给测试运行器执行的测试函数，不需要在 main 中调用。
    #[test]
    fn add_two_and_two() {
        assert_eq!(add_two(2), 4); // 比较是否相等；失败时输出两边的值。
    }

    #[test]
    fn add_two_handles_negative_input() {
        assert_eq!(add_two(-2), 0);
        assert_ne!(add_two(-2), -2); // 验证结果不等于原始输入。
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller)); // assert! 检查布尔表达式为 true。
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn rectangle_requires_both_dimensions_to_be_strictly_larger() {
        let outer = Rectangle {
            width: 8,
            height: 7,
        };
        assert!(!outer.can_hold(&Rectangle {
            width: 8,
            height: 6
        }));
        assert!(!outer.can_hold(&Rectangle {
            width: 7,
            height: 8
        }));
        assert!(!outer.can_hold(&outer));
    }

    #[test]
    fn greeting_contains_name() {
        let name = "小明";
        let result = greeting(name);
        // 失败时提供自定义信息，方便定位输入和实际输出。
        assert!(
            result.contains(name),
            "问候语缺少姓名 {name:?}，实际为 {result:?}"
        );
    }

    #[test]
    #[should_panic(expected = "猜测值必须在 1 到 100 之间")]
    fn guess_above_100_panics() {
        // 复用第 9 章的类型：发生包含指定文本的 panic 才算测试通过。
        // 如果没有 panic 或消息不匹配，测试失败。expected 是子串匹配。
        Guess::new(101);
    }

    #[test]
    #[should_panic]
    fn guess_below_1_panics() {
        // 不指定 expected 时，任意 panic 都算通过，因此通常应优先检查消息。
        Guess::new(0);
    }

    #[test]
    fn guess_accepts_boundary_values() {
        assert_eq!(Guess::new(1).value(), 1);
        assert_eq!(Guess::new(100).value(), 100);
    }

    #[test]
    fn result_test_can_use_question_mark() -> Result<(), std::num::ParseIntError> {
        // 测试也能返回 Result：Ok(()) 表示通过，Err 表示失败；? 可传播错误。
        // 返回 Result 的测试不能同时使用 #[should_panic]。
        let number = "40".parse::<i32>()?;
        assert_eq!(add_two(number), 42);
        Ok(())
    }

    #[test]
    fn unit_test_can_access_private_function() {
        assert_eq!(internal_adder(20, 22), 42);
    }

    #[test]
    fn prints_output() {
        // 成功测试的输出默认被捕获，用 -- --show-output 可以查看。
        println!("测试里的输出：{}", greeting("Rust"));
        assert_eq!(add_two(8), 10);
    }

    #[test]
    #[ignore = "演示默认忽略；可使用 -- --ignored 单独运行"]
    fn ignored_batch_example() {
        // 仅演示 ignore 标记，不故意添加耗时操作；测试没有共享可变状态。
        for number in 0..1000 {
            assert_eq!(add_two(number), number + 2);
        }
    }

    // 普通测试中，断言失败或 panic 会让测试失败，但其他测试通常继续运行。
    // 故意失败的示例保留为注释，避免默认测试集一直失败：
    // #[test]
    // fn deliberately_fails() {
    //     assert_eq!(add_two(2), 5);
    // }
}
