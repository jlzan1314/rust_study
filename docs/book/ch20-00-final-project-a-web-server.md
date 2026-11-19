# 第 20 章：写一个简单的多线程服务器

先通过 TCP 返回一个 HTTP 响应，再用固定线程池处理任务，最后安排关闭顺序。这个项目用于理解底层机制，不覆盖完整的 HTTP 协议。

- [先写一个能返回 HTTP 响应的服务器](ch20-01-single-threaded.md)
- [用固定线程池处理连接](ch20-02-multithreaded.md)
- [关闭线程池：先停止派发，再等待完成](ch20-03-graceful-shutdown-and-cleanup.md)

[返回目录](SUMMARY.md)
