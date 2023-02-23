fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let t = true;
    let f: bool = false;

    // char 类型 单引号
    let star_eye = '🤩';

    // 元组：长度固定，每个元素可不同类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);


    println!("The value of x is: {}", tup.0);

    // 数组：长度固定，每个元素相同类型
    // let a: [类型;元素数]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Wrong: let b = [i32; 5];

    let c = [3; 6];

    let first = a[0];
}
