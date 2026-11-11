pub fn run() {
    let s = String::from("hello"); // s 进入作用域

    // String 没有实现 Copy；传参时所有权移动到函数中，s 不再可用。
    // 如果需要保留 s，可以传入 s.clone() 复制内容，或改用引用借用它。
    takes_ownership(s);
    // print!("{s}");

    let x: u32 = 5; // x 进入作用域
    // u32 实现了 Copy；传参时复制值，因此之后仍可使用 x。
    makes_copy(x);
    print!("{x}");
} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: u32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
