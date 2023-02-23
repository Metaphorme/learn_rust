// 函数名使用下划线命名法：全小写，下划线分割
// 新函数可以在 main 之后
// 必须声明每个参数的类型

fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    // let x = five();

    println!("The num is {}", plus_one(3))
}

fn another_function(value: i32, unit_label: char) {
    println!("Time spend: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
