//! 13.1：闭包可以捕获定义位置的环境，类型通常由编译器推断。

use std::collections::HashMap;

/// 保存闭包，并按输入分别缓存结果；同一输入再次出现时不会重新计算。
/// 相比只存一个 Option 的简化缓存，这里不会把不同输入误认为同一个结果。
pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> {
    pub fn new(calculation: T) -> Self {
        Self {
            calculation,
            values: HashMap::new(),
        }
    }

    pub fn value(&mut self, argument: u32) -> u32 {
        // entry + or_insert_with 仅在键不存在时调用闭包。
        let calculation = &self.calculation;
        *self
            .values
            .entry(argument)
            .or_insert_with(|| calculation(argument))
    }
}

/// FnOnce 接收所有闭包：允许调用一次，也允许闭包移出捕获值。
pub fn call_once<F>(action: F) -> String
where
    F: FnOnce() -> String
{
    action()
}

/// FnMut 允许修改捕获的环境，因此调用时需要可变借用闭包。
pub fn call_twice<F: FnMut()>(mut action: F) {
    action();
    action();
}

pub fn run() {
    println!("--- 闭包 ---");
    let add_one = |number: u32| -> u32 { number + 1 };
    println!("闭包参数与返回值：{}", add_one(5));
    // 类型推断后固定：同一闭包不能先接收 String，再接收整数。
    let identity = |value| value;
    println!("类型推断：{}", identity(String::from("Rust")));

    let greeting = String::from("你好");
    // 只读取捕获值，实现 Fn；多次调用不消耗 greeting。
    let greet = |name| format!("{greeting}，{name}");
    println!("{}；{}", greet("Rust"), greet("闭包"));

    let mut count = 0;
    call_twice(|| count += 1); // 可变借用 count，实现 FnMut。
    println!("FnMut 调用次数：{count}");

    let message = String::from("闭包拥有的字符串");
    // move 把 message 的所有权移入闭包；从闭包返回它又会移出，所以只能调用一次。
    println!("FnOnce：{}", call_once(move || message));

    // move 不一定只有 FnOnce：若只读捕获值，仍可实现 Fn、重复调用。
    let label = String::from("可重复读取");
    let read = move || label.len();
    println!("move + Fn：{}，{}", read(), read());

    let mut cache = Cacher::new(|intensity| {
        println!("计算训练强度 {intensity}");
        intensity * 2
    });
    println!("首次：{}", cache.value(10));
    println!("相同输入命中缓存：{}", cache.value(10));
    println!("不同输入单独计算：{}", cache.value(20));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn caches_each_argument_separately() {
        // Cell 允许闭包通过共享引用记录次数，因此闭包仍满足 Fn。
        let calls = Cell::new(0);
        let mut cache = Cacher::new(|n| {
            calls.set(calls.get() + 1);
            n * 2
        });
        assert_eq!(cache.value(10), 20);
        assert_eq!(cache.value(20), 40);
        assert_eq!(cache.value(10), 20);
        assert_eq!(calls.get(), 2);
    }

    #[test]
    fn supports_mutation_and_moving_out() {
        let mut count = 0;
        call_twice(|| count += 1);
        assert_eq!(count, 2);
        let message = String::from("owned");
        assert_eq!(call_once(move || message), "owned");
    }
}
