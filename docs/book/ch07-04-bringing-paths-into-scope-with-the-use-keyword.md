# use：给路径一个短名字

use 把名字引入当前作用域，之后不用反复写完整路径。它不会加载另一份代码，也不会改变目标项本身的可见性。

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert("Blue", 10);
assert_eq!(scores.get("Blue"), Some(&10));
```

常见写法：

```rust
use std::collections::{HashMap, HashSet}; // 共同前缀合并。
use std::io::{self, Write}; // 同时引入 io 和 Write。
use std::fmt::Result as FmtResult; // 同名冲突时起别名。
```

`pub use` 将引入的名字再次公开，方便使用者通过简短、稳定的路径调用。普通 use 只影响当前作用域。

`use module::*` 引入所有可见项，来源容易变模糊，通常在测试等受控范围使用。添加外部依赖还需要 Cargo.toml 或 `cargo add`；只有 use 不会安装依赖。

[返回目录](SUMMARY.md)
