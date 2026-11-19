# 通道：通过发送消息协作

通道提供发送端和接收端。发送拥有的数据后，所有权转交给接收方，发送方不能继续使用被移走的值。

```rust
use std::{sync::mpsc, thread};
let (tx, rx) = mpsc::channel();
let handle = thread::spawn(move || {
    tx.send(String::from("完成")).unwrap();
});
assert_eq!(rx.recv().unwrap(), "完成");
handle.join().unwrap();
```

标准库 mpsc 表示多个发送者、一个接收者。发送端可以 clone，让多个线程发送消息；接收端不能用相同方式 clone 成多个消费者。

recv 等到消息或所有发送端关闭，try_recv 立即返回。遍历接收端会持续收取，直到所有发送端都释放；忘记释放一个发送端，可能导致接收循环一直等。

send 在接收端关闭时返回错误，recv 在通道关闭且没有剩余消息时返回错误。生产代码应根据任务需要处理，而不是统一 unwrap。

[返回目录](SUMMARY.md)
