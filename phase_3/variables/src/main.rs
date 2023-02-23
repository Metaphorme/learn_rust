fn main() {
    // 变量: 默认不可变，mut可变
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // 常量：自始至终不可变，const声明，全部大写+下划线，可进行有限运算
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3h={}s", THREE_HOURS_IN_SECONDS);

    // 遮蔽：
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value og x is: {}", y);
}
