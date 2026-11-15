# Rust 的 Package crate 和模块

结合当前 `rust_study` 项目，说明 Package、crate、模块、路径和 `use` 的关系。每个学习模块都有独立的二进制入口，可以单独运行；公共源码由库 crate 组织。

Package 是 Cargo 管理的项目单位；crate 是 Rust 的编译单元；模块组织 crate 内的代码；路径定位具体的项；`use` 将路径对应的名字引入当前作用域。

## 当前项目的结构

```text
rust_study/                    ← 一个 Package
├── Cargo.toml                 ← 包名、版本和依赖配置
└── src/
    ├── lib.rs                 ← 库 crate 的根模块
    ├── math.rs                ← 库中的 math 模块
    ├── main.rs                ← 默认二进制入口，只显示运行命令
    ├── ownership/mod.rs       ← 所有权模块
    ├── borrowing/mod.rs       ← 引用与借用模块
    ├── slice/mod.rs           ← 字符串切片模块
    ├── object_oriented/mod.rs ← 封装与 trait 模块
    ├── structs/mod.rs         ← 结构体模块
    ├── enums_and_matching/mod.rs ← 枚举与模式匹配模块
    └── bin/                   ← 每个文件都是独立的二进制 crate 入口
        ├── ownership.rs
        ├── borrowing.rs
        ├── slice.rs
        ├── object_oriented.rs
        ├── structs.rs
        ├── enums_and_matching.rs
        └── math.rs
```

上面只列出与本说明相关的文件。当前 Package 包含一个库 crate 和八个二进制 crate，其中七个用于学习示例，一个用于展示运行命令：

```text
Package rust_study
├── 库 crate，根模块为 lib.rs
│   ├── math
│   ├── ownership
│   ├── borrowing
│   ├── slice
│   ├── object_oriented
│   ├── structs
│   └── enums_and_matching
├── 二进制 rust_study，入口为 src/main.rs
├── 二进制 ownership，入口为 src/bin/ownership.rs
├── 二进制 borrowing，入口为 src/bin/borrowing.rs
├── 二进制 slice，入口为 src/bin/slice.rs
├── 二进制 object_oriented，入口为 src/bin/object_oriented.rs
├── 二进制 structs，入口为 src/bin/structs.rs
├── 二进制 enums_and_matching，入口为 src/bin/enums_and_matching.rs
└── 二进制 math，入口为 src/bin/math.rs

每个学习入口只调用对应的库模块，不会顺带运行其他学习模块。
```

`lib.rs` 和 `main.rs` 虽然位于同一个 `src` 目录，却是两个独立 crate 的入口。

## Package 和 crate 的区别

| 概念 | 作用 | 当前项目中的对应内容 |
|---|---|---|
| Package | 由 Cargo 管理构建、依赖、测试和发布 | 包含 `Cargo.toml` 的 `rust_study` 项目 |
| 库 crate | 提供供其他 crate 调用的功能 | `src/lib.rs` 及其声明的模块 |
| 二进制 crate | 生成可以运行的程序 | `src/main.rs`、`src/bin/*.rs` 各自对应一个 crate |
| 模块 | 在一个 crate 内组织代码并形成可见性边界 | `math`、`enums_and_matching` |

一个 Package 最多包含一个库 crate，可以包含多个二进制 crate，至少需要有一个 crate。额外的二进制可以放在 `src/bin/` 下，例如 `src/bin/admin.rs`。

Cargo 默认识别 `src/lib.rs`、`src/main.rs` 和 `src/bin/*.rs`，当前项目不需要额外配置 `[lib]` 或 `[[bin]]`。本项目的库 crate 名称为 `rust_study`，与 `Cargo.toml` 中的包名相同。

`src/bin/ownership.rs` 中虽然可以调用 `rust_study::ownership`，但这两者不是同一个 crate：前者是二进制入口，后者是库中的模块。

## 库 crate 示例

### 在 lib.rs 中声明公开模块

源码：[src/lib.rs](../src/lib.rs)

```rust
pub mod math;
```

> **重点：在 `src/lib.rs` 中写 `pub mod math;`，表示声明一个公开的 `math` 模块。Rust 默认会从 `src/math.rs` 或 `src/math/mod.rs` 加载这个模块的代码。**

同一条声明支持两种文件布局：

| 布局 | 模块源码位置 | `src/lib.rs` 中的声明 |
|---|---|---|
| 单文件形式 | `src/math.rs` | `pub mod math;` |
| 目录形式 | `src/math/mod.rs` | `pub mod math;` |

两种布局对外都可以通过 `rust_study::math` 访问。当前项目的 `math` 使用单文件形式，`enums_and_matching` 使用目录形式。

需要记住：

- **同一个模块的这两个位置只能选择一个。** 同时存在 `src/math.rs` 和 `src/math/mod.rs` 会导致模块文件冲突。
- **创建文件后，还需要声明模块。** 仅创建 `math.rs` 或 `math/mod.rs`，不会让它自动成为库 crate 的 `math` 模块；还要在 `lib.rs` 中写 `mod math;` 或 `pub mod math;`。
- **`pub` 控制可见性，不改变文件查找规则。** `mod math;` 和 `pub mod math;` 查找相同的两个位置，区别是后者公开模块。
- 上面的路径对应在 `src/lib.rs` 中声明根级模块的情况；嵌套模块的默认文件位置会按模块层级组织，不是所有模块都直接从 `src/` 根目录查找。

`pub` 让其他 crate 可以访问这个模块。库 crate 不需要 `main()` 函数，其入口负责组织和公开库提供的功能。

### 在 math.rs 中实现公开函数

源码：[src/math.rs](../src/math.rs)

```rust
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}
```

`add` 接收两个 `i32` 参数，返回一个 `i32`。最后一行没有分号，所以 `left + right` 的结果就是函数返回值。

这里的 `pub` 公开函数。其他 crate 要通过 `rust_study::math::add` 访问它，路径上的 `math` 模块和 `add` 函数都需要公开。

### 在二进制入口中调用库

源码：[src/bin/math.rs](../src/bin/math.rs)：

```rust
use rust_study::math::add;

fn main() {
    println!("库 crate 示例：2 + 3 = {}", add(2, 3));
}
```

路径的各部分含义是：

```text
rust_study :: math :: add
   库名       模块名   函数名
```

同一个 Package 中的二进制 crate 可以直接使用该库，不需要在 `[dependencies]` 中添加对自身 Package 的依赖。

其他二进制入口保持简短，只负责启动自己的学习模块。例如 [src/bin/ownership.rs](../src/bin/ownership.rs)：

```rust
fn main() {
    rust_study::ownership::run();
}
```

学习代码仍然保存在 `src/ownership/mod.rs` 中，由 `src/lib.rs` 的 `pub mod ownership;` 声明。切换学习主题时，直接选择不同二进制运行即可。

## mod use 和 pub 的区别

| 写法 | 作用 |
|---|---|
| `mod math;` | 声明模块，默认对父模块外部私有 |
| `pub mod math;` | 声明公开模块 |
| `use rust_study::math::add;` | 将 `add` 这个名字引入当前作用域 |
| `pub fn add(...)` | 公开函数，供模块外部通过可达路径调用 |
| `pub(crate) fn helper(...)` | 只向当前 crate 内部公开函数 |

`use` 不会改变函数的可见性，私有函数不会因为被 `use` 引用就变成公开函数。

不写 `use` 也能调用函数，只是每次都需要写完整路径：

```rust
let result = rust_study::math::add(2, 3);
```

如果目的是使用库中的 `math` 模块，二进制入口应当引用 `rust_study::math`。在 `main.rs` 中再写 `mod math;` 会把该文件作为二进制 crate 自己的模块编译，而不是导入库中的模块。

## crate self 和 super 路径

| 路径开头 | 含义 |
|---|---|
| `crate::` | 从当前 crate 的根模块开始 |
| `self::` | 从当前模块开始 |
| `super::` | 从当前模块的父模块开始 |
| `rust_study::` | 在本例的二进制中，从库 crate 开始访问 |

在库 crate 内部，可以通过以下路径调用 `add`：

```rust
crate::math::add(2, 3);
```

但在当前 `main.rs` 中，要访问库，应该写：

```rust
rust_study::math::add(2, 3);
```

原因是 `main.rs` 里的 `crate::` 指向二进制 crate 自己，而 `math` 模块声明在库 crate 中。

同理，库中的 `pub(crate)` 函数只能被该库 crate 内部使用。即使 `main.rs` 属于同一个 Package，它仍然是另一个 crate，不能调用库中仅标为 `pub(crate)` 的函数。

## 在另一个 Package 中使用这个库

假设目录结构如下，`rust_client` 与 `rust_study` 位于同一级目录：

```text
projects/
├── rust_study/
│   └── Cargo.toml
└── rust_client/
    ├── Cargo.toml
    └── src/main.rs
```

在 `rust_client/Cargo.toml` 的依赖中添加：

```toml
[dependencies]
rust_study = { path = "../rust_study" }
```

`path` 相对于声明该依赖的 `Cargo.toml` 所在目录。这里使用本地路径即可，不需要先发布库。

然后在 `rust_client/src/main.rs` 中调用：

```rust
use rust_study::math::add;

fn main() {
    println!("10 + 20 = {}", add(10, 20));
}
```

输出：

```text
10 + 20 = 30
```

## 按模块运行

在当前 `rust_study` 项目根目录运行：

```bash
cargo run
```

默认只显示各个模块的运行命令，不执行学习示例。`Cargo.toml` 中的配置明确了默认运行目标，避免多个二进制存在时无法判断运行哪一个：

```toml
[package]
name = "rust_study"
version = "0.1.0"
edition = "2024"
default-run = "rust_study"
```

根据需要，选择一个模块单独运行：

| 学习内容 | 命令 |
|---|---|
| 所有权 | `cargo run --bin ownership` |
| 引用与借用 | `cargo run --bin borrowing` |
| 字符串切片 | `cargo run --bin slice` |
| 封装与 trait | `cargo run --bin object_oriented` |
| 结构体 | `cargo run --bin structs` |
| 枚举与模式匹配 | `cargo run --bin enums_and_matching` |
| 库 crate 调用 | `cargo run --bin math` |

例如运行库调用示例：

```bash
cargo run --bin math
```

输出：

```text
库 crate 示例：2 + 3 = 5
```

一次构建全部二进制，但不执行：

```bash
cargo build --bins
```

## 添加新的学习模块

以新增 `collections` 模块为例：

1. 创建 `src/collections/mod.rs`，在其中定义 `pub fn run()` 并编写示例。
2. 在 `src/lib.rs` 中添加 `pub mod collections;`，让二进制可以访问该模块。
3. 创建 `src/bin/collections.rs` 作为独立入口：

```rust
fn main() {
    rust_study::collections::run();
}
```

然后运行 `cargo run --bin collections`。Cargo 会自动发现新入口，无需增加 `[[bin]]` 配置。可以在 `src/main.rs` 中补充一行运行提示，方便查阅。

## 构建库和查看 API 文档

只构建库，不运行二进制程序：

```bash
cargo build --lib
```

根据源码中的文档注释生成 API 文档：

```bash
cargo doc --lib --no-deps
```

生成的库文档入口为 `target/doc/rust_study/index.html`。如果想在生成后自动用浏览器打开，可以运行 `cargo doc --lib --no-deps --open`。

源码中的 `//!` 用于说明当前模块或 crate，`///` 用于说明紧随其后的函数等项，普通的 `//` 注释不会作为 API 文档正文展示。
