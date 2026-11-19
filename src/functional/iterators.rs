//! 13.2：迭代器是惰性的，适配器组合操作，消费者驱动计算。

/// 自定义迭代器，依次生成 1 到 5。
//#[derive(Default)] 把字段都实现为零值
#[derive(Default)]
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Iterator for Counter {
    type Item = u32; // 关联类型声明每次迭代产生的值。

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
            
        } else {
            None
        }
    }
}

/// 将每个整数加一，并收集为一个新 Vec。
/// ```
/// assert_eq!(rust_study::functional::iterators::increment(&[1, 2, 3]), vec![2, 3, 4]);
/// ```
pub fn increment(values: &[i32]) -> Vec<i32> {
    // iter 产生 &i32；|&value| 模式解构引用，map 产生新值，collect 消费迭代器。
    values.iter().map(|&value| value + 1).collect()
}

pub fn run() {
    println!("--- 迭代器 ---");
    let values = vec![1, 2, 3];
    let mut iterator = values.iter();
    // next 需要 &mut self，因为迭代器要记录当前位置。
    println!("手动 next：{:?}，{:?}", iterator.next(), iterator.next());
    println!("map + collect：{:?}", increment(&values));
    let total: i32 = values.iter().sum(); // sum 是消费适配器。
    println!("sum：{total}");

    let mut mutable_values = vec![1, 2, 3];
    for value in mutable_values.iter_mut() {
        *value *= 2;
    }
    println!("iter_mut 原地修改：{mutable_values:?}");
    // into_iter 消耗 Vec，产生拥有所有权的 String，不能再使用 names。
    let names = vec![String::from("Rust"), String::from("闭包")];
    let lengths: Vec<usize> = names.into_iter().map(|name| name.len()).collect();
    println!("into_iter + UTF-8 字节数：{lengths:?}");

    let mut evaluated = 0;
    let mapped = values.iter().map(|value| {
        evaluated += 1;
        value * 2
    });
    // 此时仅创建适配器，闭包还没有执行；collect 才会驱动它。
    let doubled: Vec<_> = mapped.collect();
    println!("惰性求值：{doubled:?}，实际调用 {evaluated} 次");

    // zip 得到 (1,2)、(2,3)、(3,4)、(4,5)；短的一侧结束时停止。
    let result: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|value| value % 3 == 0)
        .sum();
    println!("自定义 Iterator + zip/skip/map/filter/sum：{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_stops_and_stays_exhausted() {
        let mut counter = Counter::new();
        for expected in 1..=5 {
            assert_eq!(counter.next(), Some(expected));
        }
        assert_eq!(counter.next(), None);
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn adapters_combine_correctly() {
        let result: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|n| n % 3 == 0)
            .sum();
        assert_eq!(result, 18);
        assert_eq!(increment(&[1, 2, 3]), vec![2, 3, 4]);
        assert!(increment(&[]).is_empty());
    }

    #[test]
    fn adapters_are_lazy_and_take_limits_work() {
        let mut calls = 0;
        let mapped = (0..10).map(|n| {
            calls += 1;
            n * 2
        });
        let first: Vec<_> = mapped.take(2).collect();
        assert_eq!(first, vec![0, 2]);
        assert_eq!(calls, 2);
    }
}
