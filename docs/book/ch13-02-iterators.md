# 迭代器：逐项取值，串起处理步骤

迭代器的 next 每次返回 `Some(item)`，没有下一项时返回 None。推进迭代器通常需要可变访问。

```rust
let values = vec![1, 2, 3, 4];
let result: Vec<_> = values.iter().copied()
    .filter(|value| value % 2 == 0)
    .map(|value| value + 1)
    .collect();
assert_eq!(result, vec![3, 5]);
assert_eq!(values, vec![1, 2, 3, 4]);
```

map 转换，filter 筛选，collect 收集。iter 给引用，copied 将可 Copy 的引用目标复制出来；filter 的判断闭包会收到迭代项的引用。

适配器是惰性的：只写 map 不会执行闭包。collect、sum、for_each、for 循环或 next 开始取值时，处理才真正发生。

常见组合：zip 将两边配对，enumerate 加上下标，take 限制数量，skip 跳过前几项，fold 逐项累积。

实现自定义迭代器需要指定 Item，并实现 `next(&mut self) -> Option<Self::Item>`。普通 Iterator 不保证第一次返回 None 后永远返回 None；有这个要求时，使用 fuse 或实现 FusedIterator。

迭代器是否改变原集合，取决于它怎么借用数据以及闭包做了什么。`iter_mut().for_each(|v| *v += 1)` 就会原地修改。

[返回目录](SUMMARY.md)
