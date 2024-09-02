struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),  // [1, 5]
        6..10 => println!("one through five"), // [5, 10)
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    let Point { y: a, x: b } = p;
    assert_eq!(7, a);
    assert_eq!(0, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move{x, y} => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }

    let (first, _, third, _, fifth) = (1, 2, 3, 4, 5);

    let a = (1, 2, 3, 4, 5);;
    match a {
        (first, ..) => println!("{first}"),
    }
    let (b, ..) = a;
    println!("{b}");

    let x = 7;
    let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum AnotherMessage {
        Hello { id: i32 },
    }

    let msg = AnotherMessage::Hello { id: 5 };

    match msg {
        AnotherMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        AnotherMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
            // 错误：不能用 id
            // println!("Found an {id} in another range")
        }
        AnotherMessage::Hello { id } => println!("Found some other id: {id}"),
    }

}