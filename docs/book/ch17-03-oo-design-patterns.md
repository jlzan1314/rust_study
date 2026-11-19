# 状态模式：把允许的操作写清楚

以文章为例，状态可能是草稿、待审核、已发布。要求是：只有发布后，读者才能看到内容。

一种写法把状态存成枚举或 trait 对象，由同一个 Post 的方法判断当前能做什么。好处是外部接口简单，限制在运行时检查。

另一种写法用不同类型表示不同状态：

```rust
struct Draft { content: String }
struct Pending { content: String }
struct Published { content: String }
impl Draft {
    fn submit(self) -> Pending { Pending { content: self.content } }
}
impl Pending {
    fn approve(self) -> Published { Published { content: self.content } }
}
impl Published {
    fn content(&self) -> &str { &self.content }
}
let draft = Draft { content: String::from("文章") };
let published = draft.submit().approve();
assert_eq!(published.content(), "文章");
```

Draft 没有 content 方法，调用者不能读取未发布内容；submit 消耗草稿，避免继续把旧值当草稿使用。这叫用类型表达状态。

实际设计还要补上驳回、再次编辑、权限等业务规则。状态少且固定时枚举通常更简单；运行时需要扩展状态时，再考虑 trait 对象。

[返回目录](SUMMARY.md)
