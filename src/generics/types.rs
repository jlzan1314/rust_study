//! 10.1：泛型函数、结构体、枚举和方法。

/// T 是类型参数；PartialOrd 约束保证 T 支持比较，Copy 保证能复制元素。
/// 与书中类似，调用方必须提供非空切片，否则 list[0] 会 panic。
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// 同一个 T 要对应同一个具体类型，x 和 y 不能一个整数、一个浮点数。
struct Point<T> {
    x: T,
    y: T,
}

// impl 后声明 T，表示为所有 Point<T> 定义方法。
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x // 借用字段，不要求 T 实现 Copy。
    }
}

// 这里只为 Point<f32> 定义方法，Point<i32> 无法调用它。
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// 两个类型参数允许两个字段使用不同类型。
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    // V、W 是方法自己的类型参数，与结构体的 T、U 相互独立。
    // self 和 other 按值传入，字段被移动到新结构体中。
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    println!("\n1. 泛型：复用同一份逻辑");
    println!("最大整数：{}", largest(&[34, 50, 25, 100, 65]));
    println!("最大字符：{}", largest(&['y', 'm', 'a', 'q']));
    // 可以推断类型，也可以用 ::<i32> 主动指定函数的类型参数。
    println!("显式指定 i32：{}", largest::<i32>(&[1, 9, 3]));
    // PartialOrd 允许不可比较的值，例如 NaN；本示例使用可比较的输入。

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 3.0_f32, y: 4.0 };
    println!("整数坐标 x：{}", integer.x());
    println!("浮点坐标到原点的距离：{}", float.distance_from_origin());
    // let invalid = Point { x: 5, y: 4.0 }; // 错误：同一个 T 必须一致。

    let first = MixedPoint { x: 5, y: 10.4 };
    let second = MixedPoint { x: "hello", y: 'c' };
    let mixed = first.mixup(second);
    println!("混合坐标：({}, {})", mixed.x, mixed.y);

    // 标准库的 Option<T> 和 Result<T, E> 也是泛型枚举。
    let optional: Option<i32> = Some(42);
    let parsed: Result<i32, _> = "42".parse();
    println!("泛型枚举：{optional:?} / {parsed:?}");
    // Rust 会在编译时单态化：为实际使用的类型生成对应代码。
}
