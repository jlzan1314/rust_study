# String 和 &str：拥有文本与借用文本

String 拥有可增长的 UTF-8 文本。`&str` 只借用一段文本，字符串字面量就是一种 `&str`。

```rust
let mut text = String::from("你好");
text.push('！');
text.push_str(" Rust");
assert_eq!(text.len(), 14); // 字节数：6 + 3 + 1 + 4。
assert_eq!(text.chars().count(), 8);
```

`len()` 计算字节，不是汉字数。`chars()` 遍历 Unicode 标量值，也不保证等于屏幕上看到的字符数，例如组合字符可能由多个标量值组成。

## 拼接和读取

```rust
let left = String::from("hello");
let right = String::from("Rust");
let combined = left + " " + &right; // left 被移动，right 仍可用。
assert_eq!(combined, "hello Rust");
let another = format!("{right}!"); // format! 借用这里的参数。
assert_eq!(another, "Rust!");
```

不能写 `text[0]` 取一个字符。按字节切片时，范围必须落在 UTF-8 边界上；不确定时用 get。

```rust
let text = "你好";
assert_eq!(text.get(..3), Some("你"));
assert_eq!(text.get(..1), None);
for ch in text.chars() { println!("{ch}"); }
```

`trim()` 返回去掉两端空白后的切片，`replace()` 返回新 String，两者都不原地修改原字符串。

[返回目录](SUMMARY.md)
