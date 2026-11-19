# 共享可变数据：Arc 加 Mutex

多个线程需要修改同一份数据时，可以用 Arc 管理共享所有权，用 Mutex 保证同一时刻只有一个线程访问受保护数据。

```rust
use std::sync::{Arc, Mutex};
use std::thread;
let counter = Arc::new(Mutex::new(0));
let mut handles = Vec::new();
for _ in 0..4 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        *counter.lock().unwrap() += 1;
    }));
}
for handle in handles { handle.join().unwrap(); }
assert_eq!(*counter.lock().unwrap(), 4);
```

lock 可能等待，成功后返回锁守卫。守卫离开作用域或被 drop，锁才释放。锁内尽量只做必要操作，避免持锁等待耗时 I/O。

线程持锁时 panic 可能使锁中毒，后续 lock 返回错误，提示数据可能只更新了一半。是否恢复应根据数据状态判断。

Arc 只保证引用计数的线程安全，不会自动让内部任意类型都线程安全。多个锁还可能形成死锁，例如线程以不同顺序获取两把锁；Rust 不会自动防止这种逻辑问题。

[返回目录](SUMMARY.md)
