//! 15.3 Drop：离开作用域时清理，以及提前释放。
use std::cell::RefCell;

struct Resource<'a> {
    name: &'static str,
    events: &'a RefCell<Vec<&'static str>>,
}

impl Drop for Resource<'_> {
    fn drop(&mut self) {
        println!("释放资源：{}", self.name);
        self.events.borrow_mut().push(self.name);
    }
}

fn release_sequence() -> Vec<&'static str> {
    let events = RefCell::new(Vec::new());
    {
        let _first = Resource {
            name: "first",
            events: &events,
        };
        let _second = Resource {
            name: "second",
            events: &events,
        };
        let early = Resource {
            name: "early",
            events: &events,
        };
        // std::mem::drop 取得所有权并释放值，之后不能再使用 early。
        drop(early);
        println!("early 已提前释放，其余资源将在作用域结束时释放");
        // early.drop(); // 不允许显式调用析构方法；使用 drop(early)。
    } // 局部变量按创建顺序的逆序释放：second，再 first。
    events.into_inner()
}

pub fn run() {
    println!("\n15.3 Drop：提前释放和自动清理");
    let events = release_sequence();
    assert_eq!(events, ["early", "second", "first"]);
    println!("释放顺序：{events:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn early_drop_happens_once_before_scope_cleanup() {
        assert_eq!(super::release_sequence(), ["early", "second", "first"]);
    }
}
