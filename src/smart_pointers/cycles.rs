//! 15.6 强引用循环与 Weak 父指针。
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default)]
struct Link {
    next: RefCell<Option<Rc<Link>>>,
}

fn strong_cycle_demo() {
    let a = Rc::new(Link::default());
    let b = Rc::new(Link::default());
    *a.next.borrow_mut() = Some(Rc::clone(&b));
    *b.next.borrow_mut() = Some(Rc::clone(&a));
    assert_eq!(Rc::strong_count(&a), 2);
    assert_eq!(Rc::strong_count(&b), 2);
    println!("a、b 相互强引用，计数均为 2；只释放外部变量会留下循环");
    // 不递归打印环形结构，避免无限递归；演示后主动断环，防止真的泄漏。
    *b.next.borrow_mut() = None;
    assert_eq!(Rc::strong_count(&a), 1);
}

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<Self> {
        Rc::new(Self {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }
}

fn attach(parent: &Rc<Node>, child: &Rc<Node>) {
    *child.parent.borrow_mut() = Rc::downgrade(parent);
    parent.children.borrow_mut().push(Rc::clone(child));
}

pub fn run() {
    println!("\n15.6 引用循环：使用 Weak 表达不拥有数据的关系");
    strong_cycle_demo();
    let leaf = Node::new(3);
    assert!(leaf.parent.borrow().upgrade().is_none());
    {
        let branch = Node::new(5);
        attach(&branch, &leaf);
        println!(
            "父节点 strong = {}，weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        // upgrade 临时取得强引用，所以使用后也要释放它。
        let parent = leaf.parent.borrow().upgrade().expect("父节点仍存活");
        assert_eq!(parent.value, 5);
        println!("叶子通过 Weak 找到父节点：{}", parent.value);
    }
    assert!(leaf.parent.borrow().upgrade().is_none());
    println!("父节点释放后 upgrade = None，叶子 {} 仍然存活", leaf.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weak_parent_does_not_keep_branch_alive() {
        let leaf = Node::new(3);
        let branch = Node::new(5);
        attach(&branch, &leaf);
        assert_eq!(Rc::strong_count(&branch), 1);
        assert_eq!(Rc::weak_count(&branch), 1);
        assert!(leaf.parent.borrow().upgrade().is_some());
        drop(branch);
        assert!(leaf.parent.borrow().upgrade().is_none());
        assert_eq!(Rc::strong_count(&leaf), 1);
    }
}
