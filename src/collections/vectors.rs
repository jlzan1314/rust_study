//! 8.1：Vec<T> 动态数组。运行：cargo run --bin vectors
//! 参考：https://rustwiki.org/zh-CN/book/ch08-01-vectors.html
//!
//! Vec 可以增长或缩短，元素连续存储，且同一个 Vec 的所有元素类型必须相同。

/// 依次演示创建、修改、读取、遍历、借用和用枚举保存不同种类的数据。
pub fn run() {
    println!("\n===== 8.1：Vec<T> 动态数组 =====");
    create_and_update();
    read_elements();
    iterate_elements();
    borrowing_rules();
    store_enum_values();
}

/// 创建空 Vec，追加或弹出元素，以及使用 vec! 宏初始化。
fn create_and_update() {
    println!("\n1. 创建和更新 Vec");
    // 空集合没有元素可供推断，这里显式指定元素类型为 i32。
    let mut numbers: Vec<i32> = Vec::new();
    println!("新建空集合：{numbers:?}");

    // 修改集合需要 mut；push 将元素追加到末尾，长度随之增加。
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);
    println!("依次 push 10、20、30：{numbers:?}，长度：{}", numbers.len());

    // pop 移除并返回最后一个元素，类型为 Option<T>；空集合会返回 None。
    let last = numbers.pop();
    println!("pop 返回：{last:?}，剩余：{numbers:?}");
    let mut empty: Vec<i32> = Vec::new();
    println!("空集合 pop：{:?}", empty.pop());

    // vec! 是创建 Vec 的宏；下面的两种写法都创建三个元素。
    let values = vec![1, 2, 3];
    let repeated = vec![0; 3]; // 重复值 0 三次，不是 [0, 3]。
    println!("vec![1, 2, 3]：{values:?}");
    println!("vec![0; 3]：{repeated:?}");
    // Vec 离开作用域时，其拥有的元素和存储空间也会被释放。
}

/// 下标访问与 get 的区别：越界时，前者 panic，后者返回 None。
fn read_elements() {
    println!("\n2. 通过下标和 get 读取元素");
    let numbers = vec![10, 20, 30];
    // 索引从 0 开始；&numbers[2] 借用第三个元素，类型为 &i32。
    let third: &i32 = &numbers[2];
    println!("输入：{numbers:?}，下标 2 对应：{third}");

    for index in [1, 99] {
        // get 返回 Option<&i32>：Some 内是引用，不会取走元素的所有权。
        match numbers.get(index) {
            Some(number) => println!("get({index})：Some({number})"),
            None => println!("get({index})：None，索引不存在"),
        }
    }

    // 错误示例：下面会在运行时越界 panic，因此保留为注释。
    // let missing = &numbers[99];
}

/// &Vec 只读遍历，&mut Vec 修改元素，按值遍历则消耗 Vec。
fn iterate_elements() {
    println!("\n3. 借用遍历、修改元素和按值遍历");
    let mut numbers = vec![100, 32, 57];
    println!("输入：{numbers:?}");
    for number in &numbers {
        // number 是 &i32，读取时不用把元素移出集合。
        println!("只读遍历：{number}");
    }

    for number in &mut numbers {
        // number 是 &mut i32；* 解引用后，修改引用指向的数字。
        *number += 50;
    }
    println!("每个元素加 50：{numbers:?}");

    // 此处将 Vec 按值交给循环；循环之后不能再使用 numbers。
    for number in numbers {
        println!("按值遍历：{number}");
    }
}

/// 元素引用仍要继续使用时，不能同时对 Vec 调用需要可变借用的 push。
fn borrowing_rules() {
    println!("\n4. 元素引用与 push 的借用规则");
    let mut numbers = vec![1, 2, 3];
    let first = &numbers[0];
    println!("先使用第一个元素的引用：{first}");

    // first 最后一次使用发生在上面，不可变借用已结束，此时可以再借用为可变。
    numbers.push(4);
    println!("借用结束后 push：{numbers:?}");

    // 错误示例：如果 push 后仍要使用元素引用，会出现编译错误 E0502。
    // let first = &numbers[0];
    // numbers.push(5);
    // println!("{first}");
    // 原因：push 可能重新分配存储空间，导致原来指向元素的引用失效。
}

/// 用枚举包装不同种类的数据，Vec 的元素类型仍然只有一个。
fn store_enum_values() {
    println!("\n5. Vec 中保存不同枚举成员");
    enum Cell {
        Integer(i32),
        Decimal(f64),
        Text(String),
    }

    // 这是 Vec<Cell>，不是直接混装 i32、f64 和 String。
    let row = vec![
        Cell::Integer(3),
        Cell::Text(String::from("Rust")),
        Cell::Decimal(10.5),
    ];
    for cell in &row {
        match cell {
            Cell::Integer(value) => println!("整数单元格：{value}"),
            Cell::Decimal(value) => println!("小数单元格：{value}"),
            Cell::Text(value) => println!("文本单元格：{value}"),
        }
    }
}
