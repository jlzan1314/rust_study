pub fn run() {
    let mut s = String::from("hello");

    // 不可变引用只借用值，不取得所有权；调用后 s 仍然可用。
    let length = calculate_length(&s);
    println!("{s} 的长度是 {length}");

    // 可变引用允许修改借用的值，借用结束后可以继续使用 s。
    append_world(&mut s);
    println!("修改后: {s}");
}

fn calculate_length(value: &String) -> usize {
    value.len()
}

fn append_world(value: &mut String) {
    value.push_str(", world");
}
