# Rust 类型推断

运行：`cargo run --bin type_inference`。

[完整示例](../src/type_inference/mod.rs) 会打印值及编译器确定的类型，覆盖以下场景：

| 场景 | 类型信息来自哪里 |
|---|---|
| 字面量、运算、分支 | 字面量自身、默认数值类型、相邻表达式的类型约束 |
| Vec、HashMap | 后续的 push、insert 等使用方式 |
| 数组、元组 | 元素值及数组长度 |
| 泛型函数 | 调用参数或返回值的期望类型 |
| parse、collect、sum | 左侧标注、返回值上下文或显式泛型参数 |
| 闭包 | 函数体、调用参数或迭代器元素类型 |
| Option、Result | 内部值及上下文对各泛型参数的约束 |
| `_` 占位符 | 只标注一部分，其余由编译器推断 |

## 和前面的配置反序列化对应

```rust
let settings: ServerSettings = config.try_deserialize()?;
// 等价于：
let settings = config.try_deserialize::<ServerSettings>()?;
```

这不是隐式类型转换。左侧标注使编译器确定泛型参数 `T = ServerSettings`；运行时执行的是该类型对应的反序列化逻辑。

## 无法推断时怎么处理

下面的失败写法作为注释保留，取消注释后可以观察编译器提示。

```rust
// let port = "8080".parse().unwrap(); // E0284：不知道要解析成哪种类型
let port: u16 = "8080".parse().unwrap();
let port = "8080".parse::<u16>().unwrap();

// let items = (1..=3).collect(); // E0283：不知道要构造哪种集合
let items: Vec<_> = (1..=3).collect();

// let value = None; // E0282：不知道 Option 内部是什么类型
let value: Option<String> = None;

// let result = Ok(42); // E0282：不知道错误类型
let result: Result<_, String> = Ok(42);

// let empty = Vec::new(); // E0282：没有后续使用来确定元素类型
let empty = Vec::<u8>::new();
```

`_` 是让编译器推断的占位符，不是任意类型：`Vec<_>` 最终仍然是元素类型确定的集合。普通函数签名和结构体字段类型必须显式声明，不能像局部变量那样随意省略。

## 推断不等于自动数值转换

```rust
let small: u8 = 10;
// let large: u16 = small; // 类型不匹配，不会自动扩大整数类型
let large: u16 = u16::from(small);
```

`let large: u16 = 10;` 可以成立，是因为整数字面量 `10` 根据上下文被确定为 `u16`，而不是已经存在的 `u8` 值被隐式转换。

闭包的参数类型也会固定，不是泛型函数：

```rust
let identity = |x| x;
let first = identity(1_u8);
// let second = identity("Rust"); // 已确定参数为 u8，不能再传 &str
```

类型推断在编译期完成。编译器结合函数中的约束确定类型，不是在运行到某一行时才决定变量的类型。
