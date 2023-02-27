// 方法 与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
// 不过方法与函数是不同的，因为它们在结构体的上下文中被定义，
// 并且它们第一个参数总是 self，它代表调用该方法的结构体实例。

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 若需要获得所有权并更改
    // fn area(&mut self) -> u32 {}
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 方法的名称可以与结构中的一个字段相同
    fn width(&self) -> bool {
        self.width > 0
    }

    // 带有更多参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    // 不以 self 为第一参数的关联函数
    // 可以用来创建正方形 Rectangle 而不必两次指定同样的值
    // 通过 let sq = Rectangle::square(3); 创建
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 通过关联函数创建正方形
    let sq = Rectangle::square(3);
}
