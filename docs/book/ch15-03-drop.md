# Drop：值销毁时做清理

实现 Drop，可以在值销毁时执行清理。文件、锁、智能指针等类型通过类似机制释放资源。

```rust
struct Resource(&'static str);
impl Drop for Resource {
    fn drop(&mut self) { println!("释放 {}", self.0); }
}
{
    let _first = Resource("first");
    let _second = Resource("second");
    let early = Resource("early");
    drop(early); // 现在就释放，之后不能再用 early。
} // 接着释放 second，再释放 first。
```

局部变量通常按创建顺序的逆序销毁。结构体的字段则按声明顺序销毁，不要混为一谈。

不能显式调用 `value.drop()`。提前释放使用 `std::mem::drop(value)`，它取得值的所有权，然后结束这个值的生命期。

`drop(&value)` 只丢弃引用，不会释放它指向的数据。drop 一个 Rc 也只是减少一个强引用，最后一个强引用释放时内部值才销毁。

[返回目录](SUMMARY.md)
