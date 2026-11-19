# 条件判断和循环

`if` 的条件必须是 bool，不会把数字自动当成真假。它也能产生值，但各分支的结果类型必须一致。

```rust
let score = 80;
let passed = if score >= 60 { "通过" } else { "未通过" };
assert_eq!(passed, "通过");
```

| 写法 | 适合什么情况 |
|---|---|
| `loop` | 重复执行，直到主动 break |
| `while condition` | 条件成立就继续 |
| `for item in items` | 逐项遍历序列或区间 |

`break` 退出循环，`continue` 跳过本轮剩余部分。`loop` 可以用 `break value` 返回值。

```rust
let mut count = 0;
let result = loop {
    count += 1;
    if count == 3 { break count * 2; }
};
assert_eq!(result, 6);

for value in [10, 20, 30] {
    println!("{value}");
}
for number in (1..4).rev() {
    println!("{number}"); // 3、2、1。
}
```

`1..4` 不包含 4，`1..=4` 包含 4。遍历集合优先使用 for，通常比手动维护索引更清楚，也少一个越界风险。

[返回目录](SUMMARY.md)
