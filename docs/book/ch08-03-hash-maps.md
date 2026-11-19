# HashMap：用键找值

HashMap 适合通过姓名、编号等键查询数据。一个键最多对应一个值，重复 insert 会覆盖旧值。

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
assert_eq!(scores.insert(String::from("Blue"), 20), Some(10));
assert_eq!(scores.get("Blue"), Some(&20));
assert_eq!(scores.get("Red"), None);
```

get 返回 `Option<&V>`；get_mut 返回可变引用。String 键通常可以直接用 `&str` 查询，不必创建临时 String。

插入拥有数据的键和值，会转移它们的所有权。只插入引用则不会取得底层数据所有权，但要保证底层数据活得足够久。

## 没有就插入，有了就更新

```rust
use std::collections::HashMap;
let mut counts = HashMap::new();
for word in "Rust book Rust".split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;
}
assert_eq!(counts.get("Rust"), Some(&2));
```

entry 将“查找后插入或修改”连在一起。or_insert 返回 `&mut V`，所以可以用 `*` 修改原值。默认值计算昂贵时，用 or_insert_with，让它只在键缺失时计算。

遍历 HashMap 不保证顺序。需要按键排序时考虑 BTreeMap，或先收集后排序。哈希算法可以替换，但应按实际需求选择，不必为了入门先研究哈希实现。

[返回目录](SUMMARY.md)
