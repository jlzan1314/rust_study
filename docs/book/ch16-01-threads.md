# 线程：把任务交给另一条执行路径

`thread::spawn` 启动线程，返回 JoinHandle。调用 join 等待结束，并检查线程是否 panic。

```rust
use std::thread;
let values = vec![1, 2, 3];
let handle = thread::spawn(move || values.into_iter().sum::<i32>());
let sum = handle.join().expect("工作线程失败");
assert_eq!(sum, 6);
```

spawn 的闭包可能比当前函数活得久，因此通常用 move 将所需数据移进去。需要借用局部数据并确保线程在作用域内结束时，可以使用 thread::scope。

不 join 并不意味着工作线程一定有时间完成；主线程结束时进程会退出。多个线程的输出顺序也不固定。

并发表示任务在时间上交错推进，并行表示真的同时执行。开线程本身有成本，小任务不一定因此更快。

[返回目录](SUMMARY.md)
