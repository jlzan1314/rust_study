# 第 13 章：闭包与迭代器

参考 [Rust Book 第 13 章](book/ch13-00-functional-features.md)。在项目根目录运行：

```bash
cargo run --bin functional
cargo run --bin minigrep_iter -- to docs/poem.txt
IGNORE_CASE=1 cargo run --bin minigrep_iter -- to docs/poem.txt
```

| 文件 | 学习内容 |
|---|---|
| [closures.rs](../src/functional/closures.rs) | 闭包语法、类型推断、捕获、move、Fn trait、缓存 |
| [iterators.rs](../src/functional/iterators.rs) | 借用和所有权、惰性求值、适配器、自定义 Iterator |
| [minigrep.rs](../src/functional/minigrep.rs) | 使用迭代器解析参数和搜索文本 |
| [functional.rs](../src/bin/functional.rs) | 综合演示入口 |
| [minigrep_iter.rs](../src/bin/minigrep_iter.rs) | 迭代器版本的命令行入口 |

## 13.1 闭包：捕获环境的匿名函数

```rust
let offset = 5;
let add = |number: i32| -> i32 { number + offset };
assert_eq!(add(3), 8);
```

竖线内是参数，后面是函数体；参数和返回值类型通常可以省略。普通函数不能直接捕获局部变量，闭包可以。每个闭包表达式都有自己的匿名类型，一旦推断出参数类型，同一闭包就不能再换成其他参数类型调用。

| trait | 闭包体允许的行为 | 示例 |
|---|---|---|
| `Fn` | 不修改捕获值、不移出捕获值，可重复调用 | `|| greeting.len()` |
| `FnMut` | 可以修改捕获值，不移出捕获值 | `|| count += 1` |
| `FnOnce` | 可以移出捕获值，可能只能调用一次 | `move || message`，返回拥有所有权的 String |

所有闭包都实现 `FnOnce`，实现 `Fn` 的闭包也实现 `FnMut`。泛型边界 `F: FnOnce()` 接受三类闭包，但函数内部只能保证调用一次。

`move` 控制捕获方式，将捕获值的所有权移入闭包；它不决定闭包只能调用一次。`move || message.len()` 只读捕获的 String，仍可重复调用；`move || message` 将 String 移出，只能调用一次。

`Cacher<T: Fn(u32) -> u32>` 保存计算闭包，使用 `HashMap<u32, u32>` 分别缓存不同参数的结果。演示中输入 10 两次只计算一次，输入 20 会重新计算。缓存适合结果只由输入决定的计算；若闭包依赖变化的外部状态，缓存可能过期。

## 13.2 迭代器

| 调用方式（以 Vec 为例） | 每个元素的类型 | 对原集合的影响 |
|---|---|---|
| `iter()` | `&T` | 共享借用 |
| `iter_mut()` | `&mut T` | 可变借用，可修改元素 |
| `into_iter()` | `T` | 消耗集合，转移元素所有权 |

核心 trait 的简化形式：

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

`Some(item)` 表示下一个值，`None` 表示结束。示例 `Counter` 生成 1 到 5，结束后继续返回 `None`。一般的 Iterator trait 本身不保证返回 None 后永远结束。

- `map` 转换元素，`filter` 按条件保留元素。
- `skip(n)` 跳过前 n 项，`take(n)` 最多取 n 项。
- `zip` 配对两个序列，任意一侧结束就停止。
- `collect`、`sum` 消费迭代器，驱动前面的操作执行。

迭代器适配器是惰性的，仅构造 `map` 不会执行其中的闭包。测试使用 `map(...).take(2).collect()` 验证只处理两个元素。

综合示例先配对 `(1, 2)`、`(2, 3)`、`(3, 4)`、`(4, 5)`，再相乘得到 2、6、12、20，筛选 3 的倍数后求和得到 **18**。

## 13.3 改进 minigrep

第 12 章的 `io_project` 保留循环写法，本章通过 `functional::minigrep` 提供对照版本。

参数解析接收 `impl Iterator<Item = String>`，使用 `next()` 直接取出拥有所有权的字符串，无需先收集 Vec 或 clone。用 `ok_or(...)?` 将缺少参数时的 `None` 转成错误，同时拒绝多余参数。

搜索从“建立 Vec、遍历、条件判断、push”改为：

```rust
contents.lines().filter(|line| line.contains(query)).collect()
```

闭包捕获 `query`，结果仍然借用原始 `contents`。大小写不敏感版本在比较时转换小写，但返回原始行。两种实现的对照测试覆盖空输入、空查询、中文、CRLF 和无匹配行为。

环境变量、标准输出与标准错误、文件编码及退出状态沿用 [第 12 章说明](io_project.md)。

## 13.4 性能：循环与迭代器

迭代器是零成本抽象：编译器通常可以内联闭包并优化迭代链，性能可以接近手写循环；这不表示任意两段代码都一定一样快。`collect::<Vec<_>>()` 仍需要存储结果，`to_lowercase()` 仍会分配字符串。

性能比较应在 `--release` 下，对同一输入、同一行为重复测量，并排除终端输出和磁盘 I/O 的影响。这里提供行为等价测试，不使用一次运行耗时作为性能结论。

## 验证

```bash
cargo test --lib functional::
cargo test --doc functional::
cargo test
```
