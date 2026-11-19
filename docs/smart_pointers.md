# 第 15 章：智能指针

参考 [Rust 程序设计语言第 15 章](https://rustwiki.org/zh-CN/book/ch15-00-smart-pointers.html)。在项目根目录运行：

```bash
cargo run --bin smart_pointers
```

示例按章节顺序运行，打印关键步骤，并通过断言验证结果。源码中的错误写法保留为注释，不会中断正常运行。

| 源码 | 内容 |
|---|---|
| [boxes.rs](../src/smart_pointers/boxes.rs) | 15.1 堆分配、独占所有权、递归链表 |
| [deref.rs](../src/smart_pointers/deref.rs) | 15.2 自定义 Deref、自动解引用转换 |
| [drop_trait.rs](../src/smart_pointers/drop_trait.rs) | 15.3 自动清理、提前释放、释放顺序 |
| [reference_counting.rs](../src/smart_pointers/reference_counting.rs) | 15.4 多个所有者、强引用计数 |
| [interior_mutability.rs](../src/smart_pointers/interior_mutability.rs) | 15.5 消息记录、运行时借用、共享可变数据 |
| [cycles.rs](../src/smart_pointers/cycles.rs) | 15.6 强引用循环、Weak 父子树结构 |
| [mod.rs](../src/smart_pointers/mod.rs) | 组织模块并依次运行 |
| [smart_pointers.rs](../src/bin/smart_pointers.rs) | 独立二进制入口 |

## 普通引用与智能指针

普通引用 `&T`、`&mut T` 借用数据，通常不负责释放数据。智能指针可以在类似指针的访问方式之外，提供所有权、引用计数、自动清理等功能。前面学过的 `String` 和 `Vec<T>` 也拥有堆上的数据。

| 类型 | 所有权或用途 | 检查与限制 |
|---|---|---|
| `Box<T>` | 单个所有者，拥有堆上的值 | 普通借用规则由编译器检查 |
| `Rc<T>` | 多个所有者共享同一个值 | 单线程引用计数，共享访问本身不提供可变借用 |
| `RefCell<T>` | 单个容器拥有值，通过共享引用借出内部可变引用 | 运行时检查借用规则，不可跨线程共享 |
| `Rc<RefCell<T>>` | 多个所有者共享可修改的数据 | 单线程；可变访问仍需通过运行时借用检查 |
| `Weak<T>` | 指向 Rc 数据但不拥有数据的弱引用 | 使用前必须 `upgrade()`，目标可能已经释放 |

`RefCell` 本身不是多所有者容器；`Rc` 和 `RefCell` 解决的是不同问题。

## 15.1 Box<T>：堆上的独占所有权

```rust
let value = Box::new(5);
assert_eq!(*value, 5);
```

`Box::new` 将值放到堆上，`Box` 负责管理该值。移动 Box 会转移所有权，不会自动深拷贝内部数据。Box 离开作用域时释放其拥有的数据。

递归类型若直接包含自身，编译器无法计算它的大小：

```rust,compile_fail
enum List {
    Cons(i32, List), // 错误：递归类型有无限大小。
    Nil,
}
```

改为 `Cons(i32, Box<List>)` 后，间接指向下一个节点，大小可以确定。示例链表保存 1、2，总和为 3。Box 也适合转移较大数据的所有权，或用 `Box<dyn Trait>` 保存大小不同但实现同一 trait 的值。

Box 不会自动解决多所有者问题，也不会绕过借用规则。

## 15.2 Deref：像引用一样访问包装值

`Deref` 的核心接口是：

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

`Target` 是解引用后访问的目标类型。对于示例的包装类型，`*value` 可理解为 `*Deref::deref(&value)`；`deref` 返回引用，因此不会直接移出内部数据。教学用 `MyBox<T>` 只是元组结构体，不像标准库 Box 那样分配堆内存。

当函数要求 `&str` 时，`&MyBox<String>` 可以经过 `&String` 自动转换为 `&str`，因此可以直接传入 `greet(&name)`。这叫解引用强制转换，发生在需要匹配引用类型的位置。

| 已实现的 trait | 常见转换 |
|---|---|
| `Deref<Target = U>` | `&T` → `&U` |
| `DerefMut<Target = U>` | `&mut T` → `&mut U` |
| `Deref<Target = U>` | `&mut T` → `&U` |

不能从共享引用 `&T` 自动转换出独占的 `&mut U`。可变解引用需要实现 `DerefMut`，且仍必须符合借用规则。

## 15.3 Drop：自动清理与提前释放

实现 `Drop::drop(&mut self)` 后，值销毁时会执行自定义清理逻辑。它适合释放资源、关闭连接等场景；结构体的字段随后也会按规则自动销毁。

示例依次创建 `first`、`second`、`early`，调用 `drop(early)` 提前释放第三个值，作用域结束后按局部变量创建顺序的逆序释放其余值，记录为：

```text
["early", "second", "first"]
```

不能直接调用 `value.drop()`，应使用标准库函数 `drop(value)`（即 `std::mem::drop`）。该函数取得参数所有权，调用后不能再使用被移动的值。

注意 `drop(&value)` 只丢弃一个引用，不会销毁引用指向的值。`drop` 一个 Rc 也只释放这个所有者，是否销毁内部值还要看剩余强引用数量。

## 15.4 Rc<T>：引用计数与共享所有权

```rust
use std::rc::Rc;

let first = Rc::new(vec![1, 2, 3]);
let second = Rc::clone(&first);
assert_eq!(Rc::strong_count(&first), 2);
assert!(Rc::ptr_eq(&first, &second));
```

`Rc::clone` 增加强引用计数，共享同一份数据，不会深拷贝内部 Vec。`first.clone()` 对 Rc 也有同样效果，但 `Rc::clone(&first)` 更直观地表达“克隆智能指针”。

演示中的强引用数量依次为 **1 → 2 → 3 → 2 → 1**。最后一个强引用释放时，内部数据销毁。和普通借用不同，共享所有者中任何一个都可以先离开作用域，只要还存在强引用，数据就继续存活。

Rc 的共享访问不能直接修改普通内部值；需要共享可变数据时，可结合 RefCell。`Rc::get_mut` 则只在不存在其他 Rc 或 Weak 指向同一分配时，才能提供 `&mut T`。Rc 仅适用于单线程，跨线程引用计数通常使用 `Arc`，修改数据时还需合适的同步机制。

## 15.5 RefCell<T>：内部可变性与运行时借用

内部可变性允许类型通过共享引用提供修改内部状态的接口，但不会取消 Rust 的借用规则：**同一时刻，可以有多个共享借用，或者一个独占的可变借用。**

| 方法 | 返回类型 | 借用冲突时 |
|---|---|---|
| `borrow()` | `Ref<T>` 共享借用守卫 | panic |
| `borrow_mut()` | `RefMut<T>` 可变借用守卫 | panic |
| `try_borrow()` | `Result<Ref<T>, BorrowError>` | 返回 Err |
| `try_borrow_mut()` | `Result<RefMut<T>, BorrowMutError>` | 返回 Err |

`Ref`、`RefMut` 实现了解引用，使用起来类似引用；守卫销毁时才释放动态借用。可以使用内层作用域或 `drop(guard)` 明确结束借用，不能依赖“最后一次使用之后编译器就会销毁守卫”。

源码包含两个场景：

1. `Messenger::send(&self, ...)` 的测试替身通过 `RefCell<Vec<String>>` 记录消息；配额达到上限时发送通知，低于上限时不发送。
2. 两个 Rc 共享 `RefCell<Vec<i32>>`，一个所有者追加 3 后，另一个也能读到 `[1, 2, 3]`。持有 `borrow_mut()` 守卫期间，`try_borrow()` 返回 Err，守卫销毁后恢复读取。

```rust
use std::cell::RefCell;

let value = RefCell::new(1); // 变量绑定不需要 mut。
let reader = value.borrow();
assert!(value.try_borrow_mut().is_err());
drop(reader);
*value.borrow_mut() += 1;
assert_eq!(*value.borrow(), 2);
```

使用 RefCell 时，程序员需要保证运行时访问符合借用规则；编译通过并不代表借用操作一定不会 panic。

## 15.6 引用循环与 Weak

如果 `a` 用 Rc 拥有 `b`，`b` 也用 Rc 拥有 `a`，即使外部变量都释放，双方仍互相保持一个强引用，内部值无法销毁。这种内存泄漏可以出现在安全 Rust 中。

示例先建立循环并验证两个节点的强引用计数都为 2，然后主动清除一条边，不在演示程序中留下泄漏。也不要递归打印环形结构，否则可能因无限递归造成栈溢出。

树结构中通常采用以下关系：

```text
父节点 ── Rc（拥有）──> 子节点
父节点 <── Weak（不拥有）── 子节点
```

| 方法 | 解释 |
|---|---|
| `Rc::downgrade(&rc)` | 创建 Weak，增加弱引用计数，不增加强引用计数 |
| `weak.upgrade()` | 目标仍存活时返回 `Some(Rc<T>)`，否则返回 None |
| `Rc::strong_count(&rc)` | 当前强引用数量 |
| `Rc::weak_count(&rc)` | 当前显式弱引用数量 |

示例中，父节点持有子节点的 Rc，子节点仅保存父节点的 Weak。父节点离开作用域后可以正常销毁，外部 Rc 保持叶子存活，此时叶子查询父节点得到 None。

`upgrade()` 成功时会产生新的强引用，因此暂时保持返回值也会延长目标寿命。强引用归零后内部值销毁；只要还有 Weak，引用计数所需的分配仍可能保留，直到弱引用也释放。

## 如何选择

- 只需临时访问现有数据：优先使用 `&T` 或 `&mut T`。
- 需要独占堆上数据、构建递归类型：使用 `Box<T>`。
- 单线程多个所有者只读共享：使用 `Rc<T>`。
- 通过共享引用修改内部状态：考虑 `RefCell<T>`，明确借用守卫的作用域。
- 单线程多个所有者共享可变状态：使用 `Rc<RefCell<T>>`。
- 回指父节点等不应延长目标寿命的关系：使用 `Weak<T>`。

## 验证命令

```bash
cargo run --bin smart_pointers
cargo test --lib smart_pointers::
cargo build --bins
```

测试验证提前释放只执行一次、通知记录行为、借用守卫释放后恢复可变访问，以及 Weak 不阻止父节点销毁。
