# Result：把失败交给调用者处理

`Result<T, E>` 有两种情况：成功是 `Ok(T)`，失败是 `Err(E)`。调用可能失败的操作后，你需要决定如何处理错误。

```rust
let number: Result<i32, _> = "abc".parse();
match number {
    Ok(value) => println!("数字是 {value}"),
    Err(error) => println!("转换失败：{error}"),
}
```

遇到不同错误时，可以提示重试、采用默认值，或直接向上返回。文件不存在和权限不足通常不该用完全相同的补救方式。

## 用 ? 向上传递错误

```rust
use std::{fs, io};
fn read_name(path: &str) -> Result<String, io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(text.trim().to_owned())
}
```

`?` 遇到 Ok 就取出值继续执行；遇到 Err 就提前返回，必要时通过 From 转换错误类型。它不是忽略错误，而是让调用者接着处理。

返回 Option 的函数也能对 Option 使用 `?`，None 会提前返回。不能在返回 `()` 的普通函数里直接这样传播 Result 错误。

unwrap 遇到 Err 会 panic；expect 也会 panic，但允许附上说明。它们适合你有充分理由认为不会失败的情况，不适合替代正常错误处理。

[返回目录](SUMMARY.md)
