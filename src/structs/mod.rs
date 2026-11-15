// 定义结构体：每个字段都有名称和类型。String 由实例持有所有权。
#[derive(Debug, Clone)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 参数名与字段名相同时，可以使用字段初始化简写。
fn build_user(email: String, username: String) -> User {
    User {
        email,    // 等同于 email: email
        username, // 等同于 username: username
        active: true,
        sign_in_count: 1,
    } // 最后一个表达式没有分号，作为函数返回值
}

// 元组结构体没有字段名，通过 .0、.1 等访问；不同名称是不同类型。
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体没有字段；适合只需要类型和行为、不需要存储数据的场景。
struct AlwaysEqual;

// 结构体可以保存引用，但需要标注生命周期：引用的数据必须在使用期间有效。
//因此，UserView 实例不能比它引用的字符串 username活得更久。
struct UserView<'a> {
    username: &'a str,
}

pub fn run() {
    // 创建实例时必须提供所有字段；字段顺序可以与定义时不同。
    let mut user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };
    // 通过点号访问字段。要直接修改普通字段，整个实例绑定必须是 mut。
    user1.email = String::from("alice.new@example.com");
    println!("用户：{}，邮箱：{}", user1.username, user1.email);

    let built = build_user(String::from("bob@example.com"), String::from("bob"));
    println!("函数创建：{}", built.username);

    // 更新语法创建新实例；..user1 必须放在最后，补充未显式指定的字段。
    let user2 = User {
        email: String::from("other@example.com"),
        ..user1
    };
    // username 是 String，所有权已从 user1 移到 user2，不能再使用
    // user1.username 或将 user1 当作整体使用（会编译失败）。
    // active 和 sign_in_count 实现了 Copy，只是复制；user1.email 未被移动。
    println!("新用户：{}，原邮箱：{}", user2.username, user1.email);
    println!(
        "原状态：{}，登录次数：{}",
        user1.active, user1.sign_in_count
    );

    // 如果需要保留源实例，可以先 clone；String 的内容也会被复制。
    let user3 = User {
        email: String::from("third@example.com"),
        ..user2.clone()
    };
    println!("克隆后仍可使用：{}、{}", user2.username, user3.username);

    // 若所有非 Copy 字段都显式提供新值，更新时只复制剩余字段，源实例仍完整可用。
    let user4 = User {
        username: String::from("dave"),
        email: String::from("dave@example.com"),
        ..user3
    };
    println!("原用户：{}，新用户：{}", user3.username, user4.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("颜色：({}, {}, {})", black.0, black.1, black.2);
    let Point(x, y, z) = origin; // 元组结构体也可以解构
    println!("坐标：({x}, {y}, {z})");
    // 即使字段类型相同，Color 和 Point 也是不同的类型，不能直接互换。

    let _marker = AlwaysEqual;
    let view = UserView {
        username: &user4.username,
    };
    println!("借用的用户名：{}", view.username);
}
