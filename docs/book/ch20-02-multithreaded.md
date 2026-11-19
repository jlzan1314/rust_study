# 用固定线程池处理连接

每个连接开一个新线程容易写，但连接多了可能耗尽资源。线程池预先创建固定数量的工作线程，把任务通过通道交给它们。

下面是可独立运行的最小线程池。线程数必须大于 0；任务是取得所有权后可调用一次的闭包。

```rust
use std::{sync::{mpsc, Arc, Mutex}, thread};
type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    sender: Option<mpsc::Sender<Job>>,
    workers: Vec<thread::JoinHandle<()>>,
}
impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0, "线程数必须大于 0");
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers = (0..size).map(|_| {
            let receiver = Arc::clone(&receiver);
            thread::spawn(move || loop {
                let job = {
                    let guard = receiver.lock().expect("任务队列锁中毒");
                    guard.recv()
                }; // 执行任务前释放锁，否则其他工作线程拿不到任务。
                match job {
                    Ok(job) => job(),
                    Err(_) => break,
                }
            })
        }).collect();
        Self { sender: Some(sender), workers }
    }
    fn execute(&self, job: impl FnOnce() + Send + 'static) {
        self.sender.as_ref().expect("线程池已关闭")
            .send(Box::new(job)).expect("工作线程已全部退出");
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            if worker.join().is_err() { eprintln!("工作线程发生 panic"); }
        }
    }
}

let (tx, rx) = mpsc::channel();
{
    let pool = ThreadPool::new(2);
    for value in 0..3 {
        let tx = tx.clone();
        pool.execute(move || tx.send(value).unwrap());
    }
} // 关闭发送端，处理完已提交的任务，再等待线程退出。
drop(tx);
let mut values: Vec<_> = rx.into_iter().collect();
values.sort();
assert_eq!(values, vec![0, 1, 2]);
```

接到网络连接后，可以把上一节的 handle 函数移到同一模块，用 `pool.execute(move || { ... })` 把 TcpStream 交给任务。move 让任务拥有连接。

这里使用无界队列，任务太多仍会占用大量内存。任务 panic 还会减少存活工作线程；真正的服务还要设计队列容量、超时、故障恢复和拒绝策略。

[返回目录](SUMMARY.md)
