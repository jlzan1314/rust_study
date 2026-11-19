# 关闭线程池：先停止派发，再等待完成

顺序很关键：先让任务通道关闭，再 join 工作线程。反过来做，线程可能一直等新任务，而主线程一直等它退出，形成死锁。

上一节用 `Option<Sender<Job>>` 保存发送端，关闭时执行 `drop(self.sender.take())`。take 把 Some 中的发送端取走，并留下 None，之后就能释放它。所有发送端都释放后，接收端取完剩余任务就会返回错误，工作线程退出。

JoinHandle::join 会消耗句柄，因此用 drain 逐个取出，不能只借用句柄再 join。

用一个最小通道例子观察关闭行为：

```rust
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
tx.send(1).unwrap();
drop(tx);
assert_eq!(rx.recv().unwrap(), 1); // 已入队的消息仍能收到。
assert!(rx.recv().is_err());      // 没有发送者，也没有剩余消息。
```

服务停机还要停止接收新连接，例如收到关闭信号后退出接收循环，再销毁线程池。如果任务永久阻塞，join 也会一直等；“等待完成”本身不提供取消和超时能力。

这个项目用来串起所有权、通道、锁、trait 对象和 Drop。实际 HTTP 服务通常使用成熟框架和运行时，避免自己重复实现协议边界。

[返回目录](SUMMARY.md)
