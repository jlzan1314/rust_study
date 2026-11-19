# 变量、可变性和遮蔽

`let` 创建变量。默认不能重新赋值，需要修改就加 `mut`。

```rust
let name = "Rust";
let mut count = 1;
count += 1;
assert_eq!(count, 2);
assert_eq!(name, "Rust");
```

## 常量

`const` 用来声明编译期可以确定的值，必须写类型，名字通常全大写。不能给 const 加 mut。

```rust
const SECONDS_PER_HOUR: u32 = 60 * 60;
assert_eq!(SECONDS_PER_HOUR, 3600);
```

## 遮蔽

再次写 `let` 可以用同名的新变量替代旧变量，叫遮蔽。新变量可以换类型；重新赋值则必须保持类型不变。

```rust
let text = "123";
let text = text.len();
assert_eq!(text, 3);
```

内层作用域的同名变量只影响内层。出了花括号，外层变量仍在。

[返回目录](SUMMARY.md)
