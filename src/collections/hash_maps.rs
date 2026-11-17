//! 8.3：HashMap<K, V> 键值映射。运行：cargo run --bin hash_maps
//! 参考：https://rustwiki.org/zh-CN/book/ch08-03-hash-maps.html
//!
//! 同一个 HashMap 的键类型一致、值类型一致，每个键最多对应一个值。
//! HashMap 不保证遍历顺序，终端中键值对的排列可能随运行变化。

// Vec 和 String 通常无需导入；HashMap 需要从标准库显式引入。
use std::collections::HashMap;

/// 演示创建、查询、遍历、所有权、覆盖更新、entry 和词频统计。
pub fn run() {
    println!("\n===== 8.3：HashMap<K, V> 键值映射 =====");
    create_and_read();
    collect_pairs();
    ownership();
    update_entries();
    count_words();
}

/// insert 保存键值对，get 返回 Option<&V>，遍历时借用每一对键和值。
fn create_and_read() {
    println!("\n1. 创建、查询和遍历 HashMap");
    // 根据后面的 insert，推断为 HashMap<String, i32>。
    let mut scores = HashMap::new();
    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("黄队"), 50);
    println!("队伍分数：{scores:?}");

    for team in ["蓝队", "红队"] {
        // 即使键的类型是 String，也可以用 &str 查询，不必临时创建 String。
        match scores.get(team) {
            Some(score) => println!("查询 {team}：{score} 分"),
            None => println!("查询 {team}：队伍不存在"),
        }
    }

    // get 返回 Option<&i32>，copied 得到 Option<i32>，缺失时再使用默认值 0。
    let missing_score = scores.get("红队").copied().unwrap_or(0);
    println!("为不存在的红队提供默认分数：{missing_score}");
    for (team, score) in &scores {
        // 遍历得到 &String 和 &i32，不会取走键和值；顺序不是插入顺序。
        println!("遍历：{team} → {score}");
    }
}

/// zip 将两个序列配成键值对，collect 将这些键值对收集为 HashMap。
fn collect_pairs() {
    println!("\n2. 从两个列表收集 HashMap");
    let teams = vec![String::from("蓝队"), String::from("黄队")];
    let initial_scores = vec![10, 50];
    println!("队伍：{teams:?}，初始分数：{initial_scores:?}");

    // into_iter 消耗 teams，将每个 String 移动出来；zip 配对对应位置的分数。
    // HashMap<_, _> 指定 collect 的目标集合，键和值的具体类型由编译器推断。
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores).collect();
    println!("收集结果：{scores:?}");
    // teams 和 initial_scores 已被消耗，不能再使用；zip 在任一序列结束时停止。
}

/// 插入 String 会移动所有权；插入 Copy 类型会复制；插入引用则只借用。
fn ownership() {
    println!("\n3. HashMap 中的所有权和借用");
    let field = String::from("最喜欢的语言");
    let value = String::from("Rust");
    let mut profile = HashMap::new();
    profile.insert(field, value);
    println!("String 的所有权交给集合：{profile:?}");
    // println!("{field}：{value}"); // 编译错误 E0382：两个 String 已被移动。

    let points = 10;
    let mut scores = HashMap::new();
    scores.insert("蓝队", points); // i32 实现 Copy，复制值而不是使 points 失效。
    println!("插入后 points 仍然有效：{points}，集合：{scores:?}");

    let name = String::from("小明");
    let language = String::from("Rust");
    let mut borrowed = HashMap::new();
    borrowed.insert(name.as_str(), language.as_str()); // HashMap<&str, &str>。
    println!("只保存引用的集合：{borrowed:?}");
    println!("原来的 String 仍然有效：{name} / {language}");
    // 通过 borrowed 使用引用期间，name 和 language 的底层数据必须保持有效。
}

/// insert 会覆盖旧值；entry(...).or_insert(...) 只在键缺失时插入。
fn update_entries() {
    println!("\n4. 覆盖旧值与按需插入");
    let mut scores = HashMap::new();
    scores.insert(String::from("蓝队"), 10);
    let previous = scores.insert(String::from("蓝队"), 25);
    println!("再次插入蓝队 25 分：旧值 {previous:?}，集合 {scores:?}");

    // 蓝队已经存在，不会用 50 覆盖它；黄队不存在，所以插入 50。
    scores.entry(String::from("蓝队")).or_insert(50);
    scores.entry(String::from("黄队")).or_insert(50);
    println!("or_insert 后：{scores:?}");

    // or_insert 返回 &mut i32；可继续通过这个引用，根据旧值修改分数。
    let blue_score = scores.entry(String::from("蓝队")).or_insert(0);
    *blue_score += 5;
    println!("蓝队再加 5 分：{scores:?}");
}

/// 实际场景：统计每个单词出现的次数。
fn count_words() {
    println!("\n5. 使用 entry 统计词频");
    let text = "Rust book Rust collections Rust book";
    println!("输入：{text}");
    let mut counts = HashMap::new();

    // split_whitespace 按空白分隔，得到借用 text 的 &str，不会复制每个单词。
    for word in text.split_whitespace() {
        // 第一次出现时插入 0；再次出现时直接取得已有计数的可变引用。
        let count = counts.entry(word).or_insert(0);
        *count += 1; // count 是 &mut i32，使用 * 修改它指向的计数。
        println!("读到 {word:?} → 当前次数：{count}");
    }

    println!("词频结果：{counts:?}");
    // 使用已知的顺序逐个查询，方便对照；HashMap 自身没有排序。
    for word in ["Rust", "book", "collections"] {
        println!("{word}：{} 次", counts.get(word).copied().unwrap_or(0));
    }
}
