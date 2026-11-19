//! 15.2 Deref：自定义解引用和解引用强制转换。
use std::ops::Deref;

// 教学包装类型：内部直接保存 T，并不进行 Box 那样的堆分配。
struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn greet(name: &str) {
    println!("你好，{name}！");
}

pub fn run() {
    println!("\n15.2 Deref：像引用一样使用包装类型");
    let number = MyBox(5);
    assert_eq!(*number, 5); // 对应 *Deref::deref(&number)。
    let name = MyBox(String::from("Rust"));
    // &MyBox<String> -> &String -> &str，编译器根据参数类型自动转换。
    greet(&name);
    greet(&(*name)[..]); // 手动写出解引用和字符串切片，效果相同。
    // Deref 返回引用，不会将 String 的所有权从包装类型中移出。
}
