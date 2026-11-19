# 常见集合学习示例

参考 [Rust 程序设计语言第 8 章](book/ch08-00-common-collections.md)，示例直接写在源码中，每种集合都有独立的二进制入口。运行时会打印输入、操作过程和结果。

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

## 常用集合方法速查

下面列出学习中常用的关联函数、方法和宏，并非标准库全部 API。`T` 表示元素类型，`K` 和 `V` 表示键和值的类型；`Option<T>` 用 `Some(value)` 表示存在，用 `None` 表示不存在。标注“修改”的方法通常需要 `mut` 集合。

### Vec<T>：动态数组

| 写法 | 解释 | 是否修改或消耗原集合 |
|---|---|---|
| `Vec::new()` | 创建空数组，需要通过类型标注或后续使用确定 `T` | 创建新集合 |
| `vec![1, 2, 3]` | 用宏创建包含指定元素的数组 | 创建新集合 |
| `Vec::with_capacity(n)` | 预分配至少能容纳 `n` 个元素的空间，长度仍为 0 | 创建新集合 |
| `len()` / `is_empty()` | 返回元素数量 / 是否为空 | 否 |
| `capacity()` | 返回当前无需重新分配即可容纳的元素数量 | 否 |
| `push(value)` | 在末尾添加元素 | 修改 |
| `pop()` | 移除并返回末尾元素，返回 `Option<T>` | 修改 |
| `get(index)` | 安全读取，返回 `Option<&T>`；越界为 `None` | 否 |
| `get_mut(index)` | 安全获取可变引用，返回 `Option<&mut T>` | 可通过引用修改 |
| `first()` / `last()` | 读取首 / 尾元素，返回 `Option<&T>` | 否 |
| `insert(index, value)` | 在指定位置插入，后面的元素向后移动；`index > len()` 会 panic | 修改 |
| `remove(index)` | 移除并返回指定元素，后面的元素向前移动；越界会 panic | 修改 |
| `swap_remove(index)` | 用最后一个元素填补被移除的位置，返回被移除元素；越界会 panic，不保留顺序 | 修改 |
| `contains(&value)` | 判断是否包含某个值 | 否 |
| `sort()` | 原地升序排序，要求元素实现 `Ord` | 修改 |
| `sort_by(...)` / `sort_by_key(...)` | 按比较函数 / 提取出的键排序 | 修改 |
| `reverse()` | 原地反转顺序 | 修改 |
| `dedup()` | 删除连续重复元素；如需去除所有重复项，可先排序再调用 | 修改 |
| `retain(predicate)` | 只保留闭包返回 `true` 的元素 | 修改 |
| `extend(iter)` | 把迭代器提供的元素追加到末尾 | 修改 |
| `append(&mut other)` | 把另一个 `Vec` 的全部元素移动到末尾，`other` 变空 | 修改两个集合 |
| `truncate(n)` | 最多保留前 `n` 个元素；`n` 大于当前长度时不变 | 修改 |
| `clear()` | 删除全部元素，保留已分配容量 | 修改 |
| `drain(range)` | 移除指定范围，返回拥有被移除元素的迭代器 | 修改 |
| `as_slice()` | 获取整个数组的切片 `&[T]` | 否 |

`Vec` 可以通过解引用使用切片的方法，例如 `sort`、`contains`、`first` 和 `chunks`。`values[index]` 是索引语法，越界会 panic；索引读取非 `Copy` 元素时通常应写成 `&values[index]` 来借用。

### iter、iter_mut、into_iter：重点区别

以下以直接对 `Vec<T>` 调用为例：

| 方法 | 迭代项类型 | 含义 | 原集合能否继续使用 |
|---|---|---|---|
| `iter()` | `&T` | 共享借用元素 | 可以，但借用仍被使用时不能修改集合 |
| `iter_mut()` | `&mut T` | 可变借用元素，可通过 `*value` 修改 | 可以，但可变借用仍被使用时不能另行访问集合 |
| `into_iter()` | `T` | 消耗集合，取得元素的所有权 | 不可以，原集合已移动 |

```rust
// 共享借用：计算新集合，原集合不变。
let values = vec![1, 2, 3];
let result: Vec<_> = values.iter().map(|&value| value + 1).collect();
assert_eq!(values, vec![1, 2, 3]);
assert_eq!(result, vec![2, 3, 4]);

// 可变借用：直接修改原集合。
let mut values = vec![1, 2, 3];
for value in values.iter_mut() {
    *value += 1;
}
assert_eq!(values, vec![2, 3, 4]);

// 取得所有权：消耗原集合，再收集为新集合。
let values = vec![1, 2, 3];
let result: Vec<_> = values.into_iter().map(|value| value + 1).collect();
assert_eq!(result, vec![2, 3, 4]);
// println!("{:?}", values); // 编译错误：values 已移动。
```

`|&value|` 是参数模式匹配：把引用解构后，将整数复制给 `value`，适用于这里实现了 `Copy` 的整数。它不会修改原元素，也不能用同样的方式从 `&String` 中移动出 `String`。

`into_iter()` 消耗的是调用它的那个值。`values.into_iter()` 消耗 `Vec`，`(&values).into_iter()` 得到 `&T`，`(&mut values).into_iter()` 得到 `&mut T`。因此不能把所有 `into_iter()` 都理解成一定移动集合本身。

`for` 循环也使用 `IntoIterator`：

```rust
let mut values = vec![1, 2, 3];
for value in &values { println!("{value}"); } // 共享借用
for value in &mut values { *value += 1; }     // 可变借用
for value in values { println!("{value}"); } // 消耗 Vec
```

### Iterator：遍历、转换和收集

这些是迭代器方法，通常接在 `iter()`、`iter_mut()` 或 `into_iter()` 后面。闭包收到引用还是值，首先取决于迭代器的元素类型。

| 方法 | 解释 |
|---|---|
| `map(f)` | 将每个元素转换成新元素 |
| `filter(predicate)` | 只保留符合条件的元素；判断闭包收到的是迭代项的引用 |
| `filter_map(f)` | 转换并筛选，闭包返回 `Some(value)` 时保留，返回 `None` 时跳过 |
| `enumerate()` | 生成 `(下标, 元素)`，下标从 0 开始 |
| `zip(other)` | 将两个迭代器配对，任一迭代器结束时停止 |
| `chain(other)` | 先遍历当前迭代器，再遍历另一个 |
| `take(n)` / `skip(n)` | 最多取前 `n` 项 / 跳过前 `n` 项 |
| `rev()` | 反向遍历，要求支持双端迭代 |
| `copied()` | 将 `&T` 转为 `T`，要求 `T: Copy` |
| `cloned()` | 将 `&T` 克隆为 `T`，要求 `T: Clone` |
| `collect()` | 将元素收集成指定集合，常需要类型标注，如 `Vec<_>` 或 `HashMap<_, _>` |
| `sum()` / `product()` | 求和 / 求积，结果类型可能需要标注 |
| `fold(initial, f)` | 从初始值开始，逐项累积出结果 |
| `count()` | 消耗迭代器并返回元素数量 |
| `for_each(f)` | 对每一项执行闭包，返回 `()` |
| `any(predicate)` / `all(predicate)` | 判断是否至少一项 / 所有项满足条件，结果确定后提前停止 |
| `find(predicate)` | 返回首个满足条件的迭代项，结果为 `Option<Item>`；判断闭包收到迭代项的引用 |
| `position(predicate)` | 返回首个满足条件的位置 `Option<usize>` |
| `next()` | 推进一次，返回下一项 `Option<Item>`，需要可变访问迭代器 |

`map`、`filter` 等适配器是惰性的：只构造处理流程，直到 `collect`、`sum`、`for_each`、`for` 或 `next` 等开始取元素时才执行。`find`、`any` 等会推进迭代器，但可能提前停止。

```rust
let values = vec![1, 2, 3, 4];
let result: Vec<_> = values
    .iter()                  // 迭代项是 &i32
    .copied()                // 迭代项变成 i32
    .filter(|value| *value % 2 == 0) // filter 判断时接收 &i32
    .map(|value| value + 1)   // map 接收 i32
    .collect();
assert_eq!(result, vec![3, 5]);
assert_eq!(values, vec![1, 2, 3, 4]);
```

`collect()` 收集什么类型，由目标类型决定；创建 `HashMap` 时，迭代项必须是 `(K, V)`。调用这些方法是否影响原集合，取决于借用方式和闭包行为：例如 `iter_mut().for_each(|v| *v += 1)` 会修改原元素。

### String 与 str：字符串

`String` 提供拥有所有权、可增长的文本，很多只读方法实际来自 `str`，也可以直接在 `String` 上调用。

| 写法 | 解释 | 是否修改原字符串 |
|---|---|---|
| `String::new()` / `String::from(text)` | 创建空字符串 / 从文本创建字符串 | 创建新值 |
| `text.to_string()` | 把文本转换为拥有所有权的 `String` | 创建新值 |
| `len()` / `is_empty()` | 返回字节数 / 是否为空 | 否 |
| `push(ch)` / `push_str(text)` | 追加一个 `char` / 一段文本 | 修改 |
| `pop()` | 移除最后一个 Unicode 标量值，返回 `Option<char>` | 修改 |
| `insert(index, ch)` / `insert_str(index, text)` | 在字节位置插入字符 / 文本，位置必须在字符边界上且不越界 | 修改 |
| `remove(index)` | 移除并返回指定字节位置开始的字符，要求有效字符位置 | 修改 |
| `clear()` | 清空内容，保留容量 | 修改 |
| `get(range)` | 安全获取文本切片，返回 `Option<&str>` | 否 |
| `contains(pattern)` | 判断是否包含匹配文本 | 否 |
| `starts_with(pattern)` / `ends_with(pattern)` | 判断前缀 / 后缀 | 否 |
| `find(pattern)` | 查找首次匹配的字节位置，返回 `Option<usize>` | 否 |
| `replace(from, to)` | 替换所有匹配项，返回新的 `String` | 否 |
| `trim()` | 返回去掉两端空白后的 `&str` | 否 |
| `split(separator)` | 按分隔符返回子串迭代器 | 否 |
| `split_whitespace()` | 按连续空白拆分，忽略空白产生的空项 | 否 |
| `lines()` | 按行遍历文本 | 否 |
| `chars()` / `bytes()` | 遍历 Unicode 标量值 / 字节 | 否 |
| `char_indices()` | 遍历 `(字节位置, char)` | 否 |

`replace` 和 `trim` 不会直接改变原字符串。`String` 也没有直接的 `iter()` / `into_iter()` 字符遍历方式，应明确选择 `chars()` 或 `bytes()`。

### HashMap<K, V>：键值映射

需要 `use std::collections::HashMap;`。常见键类型需要实现 `Eq + Hash`，例如整数和 `String`。

| 写法 | 解释 | 是否修改或消耗原集合 |
|---|---|---|
| `HashMap::new()` / `HashMap::with_capacity(n)` | 创建空映射 / 预分配容量的空映射 | 创建新集合 |
| `len()` / `is_empty()` | 返回键值对数量 / 是否为空 | 否 |
| `insert(key, value)` | 插入或覆盖，返回旧值 `Option<V>` | 修改 |
| `get(&key)` | 查询值，返回 `Option<&V>` | 否 |
| `get_mut(&key)` | 查询可变引用，返回 `Option<&mut V>` | 可通过引用修改 |
| `contains_key(&key)` | 判断键是否存在 | 否 |
| `remove(&key)` | 删除键值对，返回被删除的值 `Option<V>` | 修改 |
| `entry(key)` | 获取某个键对应的条目，便于执行查询后插入或更新 | 提供修改入口 |
| `entry(key).or_insert(value)` | 键缺失时插入默认值，返回对应值的 `&mut V` | 可能修改 |
| `entry(key).or_insert_with(f)` | 键缺失时才调用闭包生成默认值，返回 `&mut V` | 可能修改 |
| `entry(key).or_default()` | 键缺失时用 `V::default()` 插入，返回 `&mut V` | 可能修改 |
| `entry(key).and_modify(f)` | 键存在时修改值，返回条目，可继续接 `or_insert` | 可能修改 |
| `keys()` / `values()` | 遍历键的引用 / 值的引用 | 否 |
| `values_mut()` | 遍历值的可变引用 | 可通过引用修改 |
| `iter()` / `iter_mut()` | 遍历 `(&K, &V)` / `(&K, &mut V)` | 后者可修改值 |
| `into_iter()` | 消耗映射，逐项取得 `(K, V)` | 消耗 |
| `retain(f)` | 根据闭包保留键值对，闭包接收 `(&K, &mut V)` | 修改 |
| `clear()` | 清空映射，保留容量 | 修改 |

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Blue"))
    .and_modify(|score| *score += 1)
    .or_insert(0);
assert_eq!(scores.get("Blue"), Some(&11));
```

当键为 `String` 时，查询通常可以直接使用 `&str`，例如 `scores.get("Blue")`，无需创建临时 `String`。`HashMap` 的遍历顺序不固定。

### 其他常见集合

这些类型位于 `std::collections`，可按用途选择：

| 类型 | 用途 | 常用方法 |
|---|---|---|
| `HashSet<T>` | 存储不重复元素，适合成员判断、去重 | `insert`、`contains`、`remove`、`intersection`、`union`、`difference` |
| `BTreeMap<K, V>` | 按键排序的映射 | `insert`、`get`、`remove`、`range`、`entry` |
| `BTreeSet<T>` | 按元素排序且去重的集合 | `insert`、`contains`、`remove`、`range` |
| `VecDeque<T>` | 双端队列，适合先进先出任务 | `push_back`、`push_front`、`pop_front`、`pop_back` |
| `BinaryHeap<T>` | 优先队列，默认优先取最大元素 | `push`、`peek`、`pop` |

`HashSet::insert` 返回是否插入了新元素；`intersection`、`union`、`difference` 分别遍历交集、并集、差集，不会修改原集合。`VecDeque` 的 `pop_front` / `pop_back` 和 `BinaryHeap` 的 `pop` 都返回 `Option<T>`。堆只保证堆顶是最大元素，普通遍历并不保证有序。

完整 API 可查阅标准库：[Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)、[String](https://doc.rust-lang.org/std/string/struct.String.html)、[HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)、[Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)、[collections](https://doc.rust-lang.org/std/collections/index.html)。
