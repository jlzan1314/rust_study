# 第 16 章：多线程协作

线程可以通过通道传消息，也可以共享被锁保护的数据。Rust 能拦下很多不安全访问，但不会自动解决死锁、任务卡住和错误的执行顺序。

- [线程：把任务交给另一条执行路径](ch16-01-threads.md)
- [通道：通过发送消息协作](ch16-02-message-passing.md)
- [共享可变数据：Arc 加 Mutex](ch16-03-shared-state.md)
- [Send 和 Sync：能不能跨线程](ch16-04-extensible-concurrency-sync-and-send.md)

[返回目录](SUMMARY.md)
