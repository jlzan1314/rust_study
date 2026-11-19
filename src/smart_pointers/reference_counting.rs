//! 15.4 Rc：单线程共享所有权。
use std::rc::Rc;

pub fn run() {
    println!("\n15.4 Rc：共享所有权与强引用计数");
    let data = Rc::new(vec![1, 2, 3]);
    println!("创建后 strong_count = {}", Rc::strong_count(&data));
    let other = Rc::clone(&data); // 只增加计数，不深拷贝内部 Vec。
    assert!(Rc::ptr_eq(&data, &other));
    println!("克隆后 strong_count = {}", Rc::strong_count(&data));
    {
        let temporary = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 3);
        println!(
            "内层作用域 strong_count = {}，值 = {temporary:?}",
            Rc::strong_count(&data)
        );
    }
    assert_eq!(Rc::strong_count(&data), 2);
    drop(other);
    assert_eq!(Rc::strong_count(&data), 1);
    println!(
        "其余所有者释放后 strong_count = {}",
        Rc::strong_count(&data)
    );
    // 最后一个强引用释放时，内部 Vec 被释放。
    // Rc 不支持跨线程共享；跨线程引用计数通常使用 Arc。
}
