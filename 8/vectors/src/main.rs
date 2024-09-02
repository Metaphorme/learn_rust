fn main() {
    let mut v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut a = vec![100, 32, 57];
    for i in &mut a {
        *i += 50;
    }

    enum SpreadSheelCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row: Vec<SpreadSheelCell> = Vec::new();
    row.push(SpreadSheelCell::Int(3));
}
