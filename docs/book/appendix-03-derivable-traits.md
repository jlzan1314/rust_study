# derive：让编译器生成常见实现

结构体和枚举可以用 derive 自动实现一部分 trait，前提是内部字段具备需要的能力。

| trait | 作用 |
|---|---|
| Debug | 使用 `{:?}` 调试输出 |
| Clone | 显式 clone，复制方式由实现决定 |
| Copy | 按位复制，赋值后旧值仍可用；还要求 Clone |
| PartialEq / Eq | 相等比较及更强的相等关系约定 |
| PartialOrd / Ord | 部分排序或全排序 |
| Hash | 将值输入哈希器，常用于哈希集合的键 |
| Default | 提供默认值 |

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Point { x: i32, y: i32 }
let first = Point { x: 1, y: 2 };
let second = first;
assert_eq!(first, second);
assert_eq!(Point::default().x, 0);
```

Copy 不能用于含 String 等非 Copy 字段的类型，也不能用于需要自定义 Drop 的类型。derive 不一定符合业务含义，例如你可能只想按用户编号比较，而不是比较全部字段；此时需要手写实现。

Display 没有通用的标准库 derive，因为面向用户的显示方式无法统一推断。

[返回目录](SUMMARY.md)
