# 写一个猜数字游戏

程序分四步：生成答案、读取输入、把字符串转成数字、比较大小。循环让用户能继续猜，猜中后退出。

先创建项目并添加随机数依赖：

```bash
cargo new guessing_game
cd guessing_game
cargo add rand@0.8
```

将下面代码放入 `src/main.rs`。这里固定使用 rand 0.8 的 API；其他版本的写法可能不同。

```rust,ignore
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let answer = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("请输入 1 到 100 之间的整数：");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("读取失败，请重试");
            continue;
        }
        let guess: u32 = match input.trim().parse() {
            Ok(value) if (1..=100).contains(&value) => value,
            _ => {
                println!("请输入有效范围内的整数");
                continue;
            }
        };
        match guess.cmp(&answer) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("猜中了！");
                break;
            }
        }
    }
}
```

`read_line` 将内容追加到 String，因此每轮重新创建空字符串。`trim` 去掉换行，`parse` 尝试转换；失败会返回 Err，程序提示后继续，不会因为输错字就崩溃。

`1..=100` 包含两端。`match` 分别处理小于、大于、等于三种情况。这里暂时照着运行，后面会逐个解释所有权、枚举和错误处理。

[返回目录](SUMMARY.md)
