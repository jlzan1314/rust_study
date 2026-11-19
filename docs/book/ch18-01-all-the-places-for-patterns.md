# 模式不只出现在 match 里

模式用来描述值的形状，并给其中的部分命名。match、if let、while let、for、let 和函数参数都能使用模式。

```rust
let point = (3, 4); // 左侧 point 本身也是模式。
let (x, y) = point;
assert_eq!(x + y, 7);
for (index, value) in [10, 20].iter().enumerate() {
    println!("{index}: {value}");
}
fn sum((a, b): (i32, i32)) -> i32 { a + b }
assert_eq!(sum((1, 2)), 3);
```

`(x, y)` 表示一个二元组，并把两个位置分别绑定为 x、y。解构结构体、枚举时也是同样思路。

if let 可以只处理匹配成功的情况；let 普通绑定则要求模式一定能匹配，见下一节。

[返回目录](SUMMARY.md)
