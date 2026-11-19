# 模式是否可能匹配失败

“不可反驳模式”就是一定能匹配；“可反驳模式”就是可能失败。不需要记术语，先看例子。

`let (x, y) = (1, 2)` 一定成功。`Some(x)` 不一定成功，因为值可能是 None，所以不能直接写 `let Some(x) = value;`。

可能失败时，用 if let、match，或 let-else：

```rust
fn print_number(value: Option<i32>) {
    let Some(number) = value else {
        println!("没有数字");
        return;
    };
    println!("数字是 {number}");
}
print_number(Some(3));
print_number(None);
```

let-else 的 else 分支必须离开当前流程，例如 return、break 或 panic。成功后绑定的 number 可以继续在后面使用。

普通 let、函数参数和 for 中的模式通常要求一定能匹配；match 等条件位置可以处理可能失败的模式。

[返回目录](SUMMARY.md)
