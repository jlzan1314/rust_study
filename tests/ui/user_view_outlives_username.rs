// 复用项目中的 UserView 定义，使测试跟随实际类型的变化。
include!("../../src/structs/mod.rs");

fn main() {
    let user;

    {
        let username = String::from("小明");
        user = UserView {
            username: &username,
        };
    } // username 在这里被销毁，user.username 随之失效。

    println!("{}", user.username); // 预期报错 E0597：借用的数据存活时间不够长。
}
