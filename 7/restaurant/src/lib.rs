mod front_of_house {
    pub mod hosting {
        const A: i32 = 3;
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {
            super::show();  // 调父模块
        }

        fn serve_order() {}

        fn take_payment() {}
    }

    // 错误：父调不了子
    // fn test() { serving::take_order() }

    fn show() {}
}

mod back_of_house {
    pub struct Breakfask {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {  // 对比以上，枚举 pub 后所有成员 pub
        Soup,
        Salad,
    }

    impl Breakfask {
        pub fn summer(toast: &str) -> Breakfask {
            Breakfask {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

use crate::front_of_house::hosting;
// 公共 API
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfask::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();

}
