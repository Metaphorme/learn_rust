#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of rectangle is {} square pixels.",
        area(&rect1)
    );

    // 打印 Rectangle 实例
    println!("rect1 is {:?}", &rect1);
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
