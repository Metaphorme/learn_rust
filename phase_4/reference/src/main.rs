fn main() {
    // 引用
    // 创建一个"引用"的行为叫"借用"
    let s1 = String::from("Hello");
    let len = caculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // 可变引用
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    // 在同一时间，只能有一个对某一特定数据的可变引用。
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2)

    // 可以不同时引用：
    let r1 = &mut s;
    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);

    // 或者创建一个新的作用域：
    {
        let r3 = &mut s;
    }   // r3 被丢弃
    // println!("{}", r3);

    let r4 = &mut s;
    println!("{}", r4);

    // 不能同时使用可变和不可变引用
    // let r5 = &s;
    // let r6 = &s;
    // let r7 = &mut s;
    // println!("{}, {}, and {}", r5, r6, r7);

    // 多个不可变引用是可以的

    // 注意：引用的作用域从声明的地方开始一直持续到最后一次使用为止
    // 最后一次使用不可变引用（println!)，发生在声明可变引用之前，所以如下代码是可以编译的
    // 作用域不重叠，非词法作用域生命周期

    let r8 = &s; // 没问题
    let r9 = &s; // 没问题
    println!("{} and {}", r8, r9);
    // 此位置之后 r1 和 r2 不再使用

    let r10 = &mut s; // 没问题
    println!("{}", r10);

    // 悬垂引用
    let reference_to_nothing = dangle();


}

fn caculate_length(s: &String) -> usize {
    s.len()
}

// 引用是不可变的
//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");
    // &s
    s
} // 这里 s 离开作用域，被丢弃

// 原则：
//   在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
//   引用必须总是有效的。