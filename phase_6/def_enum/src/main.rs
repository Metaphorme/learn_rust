enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr1 {
    V4(String),
    V6(String),
}

// 用枚举替代结构体还有另一个优势：
// 每个成员可以处理不同类型和数量的数据。
// IPv4 版本的 IP 地址总是含有四个值在 0 和 255 之间的数字部分。
// 如果我们想要将 V4 地址存储为四个 u8 值而 V6 地址仍然表现为一个 String，这就不能使用结构体了。
// 枚举则可以轻易地处理这个情况：

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体。
// 甚至可以包含另一个枚举

enum Message {
    Quit,                           // 没有关联任何数据
    Move { x: i32, y: i32},         // 匿名结构体
    Write(String),                  // 单独 String
    ChangeColor(i32, i32, i32),     // 三个 i32
}

// 可以在 Message 枚举上定义一个叫做 call 的方法
// 与 Struct 相同，都使用 impl 关键字

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Option 枚举
// <T> 泛型类型参数，意味着 Option 枚举的 Some 成员可以包含任意类型的数据
// enum Option<T> {
//    Some(T),
//    None,
// }


// 定义函数
fn route(ip_type: IpAddrKind) {}

fn main() {
    // 枚举值
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // 使用结构体将枚举成员与值相关联
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 仅仅使用枚举并将数据直接放进每一个枚举成员
    let home1 = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr1::V6(String::from("::1"));

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));

    // 调用枚举的方法体
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option 枚举
    // <T> 泛型类型参数，意味着可以包涵任意类型的数据
    let some_number = Some(5);
    let some_string = Some("a string");

    // 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，
    // 因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
    let x: Option<u32> = None;
    // 当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。
    // 当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // no implementation for `i8 + std::option::Option<i8>`
    // let sum = x + y;
    // 为了拥有一个可能为空的值，你必须要显式地将其放入对应类型的 Option<T> 中。
    // 接着，当使用这个值时，必须明确地处理值为空的情况。
    // 只要一个值不是 Option<T> 类型，你就可以安全地认定它的值不为空。
    // 这是 Rust 的一个经过深思熟虑的设计决策，来限制空值的泛滥以增加 Rust 代码的安全性。
}
