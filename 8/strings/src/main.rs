fn main() {
    // let mut s = String::new();

    let data = "initial contents";

    let mut s = data.to_string();

    let s2 = "Ha";
    // s.push_str(&s2);
    s.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    let a = "你好".to_string();
    for c in a.chars() {
        println!("{c}");
    }

    for b in a.bytes() {
        println!("{b}");
    }

}
