fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");  // 无序
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{field_name}");  // 无效

    // 覆盖
    scores.insert(String::from("Blue"), 15);
    {}
    scores.insert(String::from("Blue"), 25);

    scores.entry(String::from("Red")).or_insert(70);

    // 旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // count 是值的可变引用
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}
