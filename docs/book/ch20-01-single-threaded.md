# 先写一个能返回 HTTP 响应的服务器

TCP 提供字节流，HTTP 规定请求和响应怎样组织。下面的程序只处理最简单的请求行，访问 `/` 返回 200，其他路径返回 404。

把代码放到单独二进制入口后运行，再用浏览器访问 `http://127.0.0.1:7878/`。

```rust,no_run
use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut request_line = String::new();
    {
        let mut reader = BufReader::new(&mut stream);
        reader.read_line(&mut request_line)?;
    }
    let (status, body) = if request_line.trim_end() == "GET / HTTP/1.1" {
        ("200 OK", "你好，Rust！")
    } else {
        ("404 Not Found", "页面不存在")
    };
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) = handle(stream) { eprintln!("请求失败：{error}"); }
            }
            Err(error) => eprintln!("连接失败：{error}"),
        }
    }
    Ok(())
}
```

响应头用 `\r\n` 换行，头和正文之间空一行。Content-Length 计算字节数，所以这里用 body.len()。write_all 尝试写完全部响应，普通 write 可能只写一部分。

这个服务器一次只处理一个连接。某个客户端迟迟不发送完整请求行，后面的连接就得等。它也没有完整解析请求头、请求体、连接复用和超时，只适合学习，不能直接当完整 HTTP 服务使用。

[返回目录](SUMMARY.md)
