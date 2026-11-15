// Rust 没有 class 关键字：结构体保存数据，impl 块定义关联函数和方法。
// 字段默认对模块外私有；调用者通过公开方法访问和修改数据，这就是封装。
pub struct BankAccount {
    owner: String,
    balance: u32,
}

impl BankAccount {
    // 没有 self 的函数是关联函数，常用于构造实例，通过 :: 调用。
    pub fn new(owner: &str, balance: u32) -> Self {
        Self {
            owner: owner.to_string(),
            balance,
        }
    }

    // &self 不可变借用实例，可以读取字段而不转移所有权。
    pub fn balance(&self) -> u32 {
        self.balance
    }

    // &mut self 可变借用实例，可以修改字段。
    pub fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }
}

// trait 定义共同的行为；不同的结构体可以分别实现它。
trait Describe {
    fn describe(&self) -> String;
}

impl Describe for BankAccount {
    fn describe(&self) -> String {
        format!("{} 的账户余额：{}", self.owner, self.balance)
    }
}

struct Student {
    name: String,
}

impl Describe for Student {
    fn describe(&self) -> String {
        format!("学生：{}", self.name)
    }
}

// &dyn Describe 是 trait 对象：运行时根据实际类型选择对应的方法。
// Rust 没有传统的类继承；可以通过组合结构体和实现 trait 来复用及共享行为。
fn print_description(value: &dyn Describe) {
    println!("{}", value.describe());
}

pub fn run() {
    let mut account = BankAccount::new("小明", 100);
    account.deposit(50);
    println!("查询余额：{}", account.balance());

    let student = Student {
        name: String::from("小红"),
    };
    print_description(&account);
    print_description(&student);
}
