//! 15.1 Box：独占所有权与递归类型。

#[derive(Debug)]
enum List {
    // Box 的大小固定，因此编译器可以确定 List 的大小。
    // Cons(i32, List) 则会形成无限大小的递归类型，无法编译。
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn sum(&self) -> i32 {
        match self {
            Self::Cons(value, next) => value + next.sum(),
            Self::Nil => 0,
        }
    }
}

pub fn run() {
    println!("\n15.1 Box：堆分配与递归类型");
    let number = Box::new(5);
    println!("Box 中的值：{}，解引用：{}", number, *number);
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    assert_eq!(list.sum(), 3);
    println!("递归链表：{list:?}，总和：{}", list.sum());
    // number 离开作用域后自动释放拥有的数据，无需手动 free。
}
