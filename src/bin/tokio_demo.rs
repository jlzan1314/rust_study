//! 运行 `cargo run --bin tokio_demo`。

use std::{error::Error, time::Duration};
use tokio::{sync::mpsc, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("1. 用 await 等待异步操作");
    println!("{}", work("准备", 50).await);

    println!("\n2. 用 join! 在当前任务中并发等待两个 Future");
    let (first, second) = tokio::join!(work("查询 A", 100), work("查询 B", 50));
    println!("结果：{first}、{second}");

    println!("\n3. 用 spawn 启动任务，用 mpsc 传递消息");
    send_messages().await?;

    println!("\n4. 用 timeout 限制等待时间");
    match time::timeout(Duration::from_millis(50), work("慢查询", 200)).await {
        Ok(result) => println!("结果：{result}"),
        Err(_) => println!("等待超时，慢查询的 Future 已丢弃"),
    }
    Ok(())
}

async fn work(name: &str, millis: u64) -> String {
    println!("开始{name}");
    // 异步等待期间，运行时可以执行其他任务。
    time::sleep(Duration::from_millis(millis)).await;
    println!("结束{name}");
    format!("{name}完成")
}

async fn send_messages() -> Result<(), Box<dyn Error>> {
    // 缓冲区最多存放一条消息，满时 send().await 等待接收端腾出空间。
    let (tx, mut rx) = mpsc::channel(1);
    let producer = tokio::spawn(async move {
        for message in ["第一条消息", "第二条消息"] {
            tx.send(message).await?;
        }
        // 任务结束，唯一的发送端 tx 被丢弃。
        Ok::<(), mpsc::error::SendError<&str>>(())
    });

    while let Some(message) = rx.recv().await {
        println!("收到：{message}");
    }
    // 通道关闭且消息已读完后，recv 返回 None。
    // 第一个 ? 处理任务执行错误，第二个 ? 处理发送错误。
    producer.await??;
    Ok(())
}
