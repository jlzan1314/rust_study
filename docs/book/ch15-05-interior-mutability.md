# RefCell：把借用检查放到运行时

有时接口只能提供 `&self`，但内部确实需要记录或更新状态，可以用 RefCell。它允许通过共享引用访问内部可变数据，并在运行时检查借用冲突。

```rust
use std::cell::RefCell;
let value = RefCell::new(1);
let reader = value.borrow();
assert!(value.try_borrow_mut().is_err());
drop(reader);
*value.borrow_mut() += 1;
assert_eq!(*value.borrow(), 2);
```

规则没有变：同时可以有多个共享借用，或者一个独占可变借用，不能两者并存。

| 方法 | 冲突时怎么办 |
|---|---|
| borrow / borrow_mut | panic |
| try_borrow / try_borrow_mut | 返回 Err |

borrow 返回 Ref 守卫，borrow_mut 返回 RefMut 守卫。守卫销毁时才释放借用。用内层花括号或 drop 明确结束；不要以为最后一次使用后守卫就一定提前销毁。

常见用途是测试替身：send 接收 `&self`，内部用 `RefCell<Vec<String>>` 记录消息。多个所有者还需要共享修改时，组合为 `Rc<RefCell<T>>`。

RefCell 不提供线程同步。跨线程共享可变状态通常需要 Mutex 或 RwLock。

[返回目录](SUMMARY.md)
