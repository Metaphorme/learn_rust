struct User {
    // 每一部分数据的名字和类型，称为 字段（field）
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 没有命名字段元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 没有任何字段的类单元结构体
struct  AlwaysEqual;

fn main() {
    // 实例中字段的顺序不需要和它们在结构体中声明的顺序一致

    // 不可变：
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 可变：
    // 整个实例必须可变，不能某字段可变
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    // 结构体更新语法：
    let user3 = User {
        email: String::from("another@example.com"),
        // ..user1 必须放在最后
        ..user1
    };

    // 在这个例子中，我们在创建 user3 后不能再使用 user1，因为 user1 的 username 字段中的 String 被移到 user3 中。
    // 如果我们给 user3 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user3 后仍然有效。active 和 sign_in_count 的类型是实现 Copy trait 的类型。

    // 元组结构体
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 类单元结构体
    // 每个实例始终等于每一个其他类型的实例
    let subject = AlwaysEqual;

}

// 函数体返回新实例
fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        // 若参数名和字段名完全相同：
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
