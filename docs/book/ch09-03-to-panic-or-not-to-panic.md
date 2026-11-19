# 什么时候返回错误，什么时候 panic

判断依据是：调用者有没有合理的处理办法。

| 情况 | 通常怎么做 |
|---|---|
| 用户输入不合法、网络超时、文件缺失 | 返回 Result，让调用者决定 |
| 违反程序内部约定，继续执行会产生错误结果 | 可以 panic |
| 测试要验证结果 | 使用断言，失败时 panic |
| 临时原型 | 可以先 expect，但要写明假设 |

库函数通常优先返回错误，让使用者决定重试、记录还是终止。公开函数可能 panic 时，应在文档里写清条件。

## 用类型挡住无效数据

```rust
struct Guess(u8);
impl Guess {
    fn new(value: u8) -> Result<Self, &'static str> {
        if (1..=100).contains(&value) {
            Ok(Self(value))
        } else {
            Err("数字必须在 1 到 100 之间")
        }
    }
    fn value(&self) -> u8 { self.0 }
}
assert_eq!(Guess::new(50).unwrap().value(), 50);
assert!(Guess::new(0).is_err());
```

字段私有，外部只能通过检查后的构造方法创建值。后续函数接收 Guess，就不必每次重复检查范围。

[返回目录](SUMMARY.md)
