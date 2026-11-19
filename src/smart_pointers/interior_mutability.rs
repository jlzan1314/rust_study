//! 15.5 RefCell：通过共享引用修改内部值，在运行时检查借用。
use std::cell::RefCell;
use std::rc::Rc;

trait Messenger {
    fn send(&self, message: &str);
}

#[derive(Default)]
struct MockMessenger {
    // send 只能得到 &self，使用 RefCell 记录收到的消息。
    messages: RefCell<Vec<String>>,
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.messages.borrow_mut().push(message.to_owned());
    }
}

fn notify_usage(messenger: &impl Messenger, used: usize, limit: usize) {
    if used >= limit {
        messenger.send("配额已用尽");
    }
}

pub fn run() {
    println!("\n15.5 RefCell：内部可变性与借用守卫");
    let mock = MockMessenger::default(); // 绑定无需 mut。
    notify_usage(&mock, 100, 100);
    println!("通过 &self 记录消息：{:?}", mock.messages.borrow());

    let shared = Rc::new(RefCell::new(vec![1, 2]));
    let other = Rc::clone(&shared);
    {
        let mut values = other.borrow_mut(); // RefMut 守卫管理可变借用。
        values.push(3);
        // 可变借用期间不能再借用，try_borrow 返回 Err，不会 panic。
        assert!(shared.try_borrow().is_err());
        // shared.borrow(); // 若取消注释，这里会因借用冲突而 panic。
    } // RefMut 被释放，借用结束；仅“最后一次使用”不保证守卫提前释放。
    assert_eq!(*shared.borrow(), vec![1, 2, 3]);
    println!("两个所有者看到相同修改：{:?}", shared.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_reference_can_record_notifications() {
        let mock = MockMessenger::default();
        notify_usage(&mock, 99, 100);
        assert!(mock.messages.borrow().is_empty());
        notify_usage(&mock, 100, 100);
        assert_eq!(*mock.messages.borrow(), ["配额已用尽"]);
    }

    #[test]
    fn borrow_is_available_again_after_guard_is_dropped() {
        let cell = RefCell::new(1);
        let reader = cell.borrow();
        assert!(cell.try_borrow_mut().is_err());
        drop(reader);
        *cell.borrow_mut() = 2;
        assert_eq!(*cell.borrow(), 2);
    }
}
