#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // match 有些像 if
    // 对于 if，表达式必须返回一个布尔值，而 match 它可以是任何类型的
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1       // return 1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // 绑定值的模式
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// 匹配 Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    // 匹配 Some(T)
    let six = plus_one(five);
    let none = plus_one(None);

    // 通配模式和 _ 占位符
    let dice_roll = 9;
    match dice_roll {
        3 => add_funcy_hat(),
        7 => remove_fancy_hat(),
        // 当我们不想使用通配模式获取的值时，请使用 _ ，
        // 这是一个特殊的模式，可以匹配任意值而不绑定到该值
        // 满足穷举性要求
        // _ => reroll(),

        // 如果你掷出 3 或 7 以外的值，你的回合将无事发生。
        // 我们可以使用单元值（在“元组类型”一节中提到的空元组）作为 _ 分支的代码
        // - => (),

        // 必须将通配分支放在最后，因为模式是按顺序匹配的
        other => move_player(other),
    }
}
