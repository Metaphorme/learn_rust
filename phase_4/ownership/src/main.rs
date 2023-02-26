fn main() {
    // 字符串字面量
    let sa = "Hello!";
    println!("{}", sa);


    // String 类型
    let mut st = String::from("Hello");
    st.push_str(", world!");
    println!("{}", st);
    // 本身不可追加
    // let st = String::from("Hello");
    // st.push_str(", world!");

    // 数据交互方式一：移动
    // 带有 Copy trait 的标注
    // 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用
    // 如下是一些 Copy 的类型：
    // 所有整数类型，比如 u32。
    //   布尔类型，bool，它的值是 true 和 false。
    //   所有浮点数类型，比如 f64。
    //   字符类型，char。
    //   元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

    // x,y 都放在了栈上
    let x = 5;
    let y = x;
    println!("x={}, y={}", x, y);

    // 放在堆上
    // Emp.1
    // s1 已被释放，出错
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1)

    // 数据交互方式二：克隆
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 所有权和函数

    let s = String::from("hello");
    takes_ownership_string(s);

    // 此时，s已被释放
    // println!("{}", s)

    let m = 5;
    takes_ownership_integer(m);

    // 此时，m已被释放，但 i32 是 Copy 的，所以之后仍可以使用 m
    println!("{}", m);

    // 返回值与作用域
    // 返回值也可以转移所有权

    let a1 = gives_ownership();
    println!("{}", a1);

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);

}

fn takes_ownership_string(some_string: String) { // some_string 进入作用域
    println!("{}", some_string)
}  // 这里，some_string 移出作用域并释放

fn takes_ownership_integer(some_integer: i32) { // some_string 进入作用域
    println!("{}", some_integer)
}  // 这里，some_integer 移出作用域并释放

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    // gives_ownership 将返回值移动给调用它的函数
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

