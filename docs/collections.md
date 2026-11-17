# 常见集合学习示例

参考 [Rust 程序设计语言第 8 章](https://rustwiki.org/zh-CN/book/ch08-00-common-collections.html)，示例直接写在源码中，每种集合都有独立的二进制入口。运行时会打印输入、操作过程和结果。

## 源码与运行命令

| 主题 | 源码 | 单独运行 |
|---|---|---|
| Vec 动态数组 | [vectors.rs](../src/collections/vectors.rs) | `cargo run --bin vectors` |
| String 与 UTF-8 | [strings.rs](../src/collections/strings.rs) | `cargo run --bin strings` |
| HashMap 键值映射 | [hash_maps.rs](../src/collections/hash_maps.rs) | `cargo run --bin hash_maps` |

模块入口为 [src/collections/mod.rs](../src/collections/mod.rs)。三个子模块由库 crate 公开，各二进制只调用自己的 `run()`：

```rust
fn main() {
    rust_study::collections::vectors::run();
}
```

这里的 `rust_study` 是库 crate，`collections` 和 `vectors` 是它的模块，`vectors` 二进制是另一个独立的 crate。

## Vec 动态数组

`Vec<T>` 保存数量可变、类型相同的元素。源码依次演示：

1. 用 `Vec::new()`、`vec![1, 2, 3]` 和 `vec![0; 3]` 创建集合。
2. 用 `push` 追加元素，用 `pop` 移除末尾元素；空集合的 `pop` 返回 `None`。
3. 用下标或 `get` 读取元素；下标越界会 panic，`get` 越界返回 `None`。
4. 用 `&Vec` 只读遍历，用 `&mut Vec` 配合解引用修改元素。
5. 解释元素引用与 `push` 的借用冲突，以及用枚举包装不同种类的数据。

重点：`get` 返回 `Option<&T>`，其中保存的是元素引用。引用还要继续使用时，不能同时修改集合；`push` 可能导致存储空间重新分配。

用枚举保存整数、浮点数和文本时，集合的实际类型仍是 `Vec<Cell>`，每个元素都是同一个枚举类型。

## String 与 UTF-8

`String` 拥有字符串数据，`&str` 借用一段文本。源码依次演示：

1. 用 `String::new()`、`String::from()` 和 `to_string()` 创建字符串。
2. `push` 追加一个 `char`，`push_str` 追加一段 `&str`。
3. `left + &right` 移动左侧 `String` 的所有权，借用右侧文本。
4. `format!` 借用格式化参数，并返回新的 `String`。
5. 比较字节数与 Unicode 标量值数量，展示安全切片和字符遍历。

重点：字符串的 `len()` 和切片范围以字节计算，不是按汉字数量计算。

```rust
let text = "你好Rust";

println!("{}", text.len());           // 10 个字节
println!("{}", text.chars().count()); // 6 个 Unicode 标量值
println!("{}", &text[0..3]);          // "你"
println!("{:?}", text.get(0..1));     // None，范围切断了一个汉字
```

单个整数下标如 `text[0]` 无法编译；范围切片如 `&text[0..1]` 可以通过类型检查，但会因为 UTF-8 边界无效而在运行时 panic。范围不确定时，可以用 `get` 接收 `Option<&str>`。

`chars()` 遍历 Unicode 标量值，`bytes()` 遍历原始字节，`char_indices()` 同时提供字节起始位置和标量值。组合字符可能由多个标量值构成，因此 `chars().count()` 不是通用的可见字符计数。

## HashMap 键值映射

`HashMap<K, V>` 通过键查找值，同一个键最多对应一个值。源码依次演示：

1. 导入 `std::collections::HashMap`，用 `new` 和 `insert` 保存队伍分数。
2. 用 `get` 查询，处理键存在和缺失的情况，并借用遍历键值对。
3. 用 `into_iter`、`zip` 和 `collect` 将两个列表收集为键值映射。
4. 对比插入 `String`、`i32` 和引用时的所有权行为。
5. 用 `insert` 覆盖旧值，用 `entry(...).or_insert(...)` 按需插入并修改值。
6. 按空白拆分文本，使用 `entry` 统计单词出现次数。

重点：`get` 返回 `Option<&V>`；`or_insert` 返回 `&mut V`，可以通过 `*` 修改原来的值。

```rust
use std::collections::HashMap;

let mut counts = HashMap::new();
for word in "Rust book Rust collections Rust book".split_whitespace() {
    let count = counts.entry(word).or_insert(0);
    *count += 1;
}
```

结果中 `Rust` 出现 3 次，`book` 出现 2 次，`collections` 出现 1 次。HashMap 不保证遍历顺序，打印时键值对的位置可能变化。

## 错误示例的查看方式

会导致编译失败或运行时 panic 的代码保留为源码注释，旁边说明原因和正确写法。需要观察具体错误时，可以单独取消相应片段的注释，再运行对应二进制。
