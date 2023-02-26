fn main() {
    // Slice: 无所有权，允许引用集合中的一段连续的元素序列，而不是引用整个集合
    let mut s = String::from("hello world");
    let word = first_word(&s); // word 为 5
    println!("{}", word);
    s.clear(); // 清空s

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效（不同步）！

    let hello = &s[0..5];
    // 等效 let hello = &s[..5];

    let world = &s[6..11];
    // 等效 let hello = &s[6..];

    // 全字符串 Slice
    // let hello_world = &s[..]

    let mut s = String::from("hello world");
    let word = first_word_new(&s);
    // s.clear();
    // 当拥有某值的不可变引用时，就不能再获取一个可变引用。
    // 因为 clear 需要清空 String，它尝试获取一个可变引用。在调用 clear 之后的 println! 使用了 word 中的引用，所以这个不可变的引用在此时必须仍然有效。
    // Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败。
    println!("The first word is: {}", word);

    // 字符串字面量就是 Slice
    let s1 = "Hello, world!";
    print_type_of(&s1);

    let my_string = String::from("hello world");
    // `first_word` 接受 `String` 的切片，无论是部分还是全部
    let word = first_word_new(&my_string[0..6]);
    let word = first_word_new(&my_string[..]);
    // `first_word` 也接受 `String` 的引用，
    // 这等同于 `String` 的全部切片
    let word = first_word_new(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 接受字符串字面量的切片，无论是部分还是全部
    let word = first_word_new(&my_string_literal[0..6]);
    let word = first_word_new(&my_string_literal[..]);

    // 因为字符串字面值**就是**字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word_new(my_string_literal);

    // 其他类型的 slice
    let a = [1, 2, 3, 4, 5];
    let s2 = &a[1..3];

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// 习题：编写一个函数，该函数接收一个字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // 只需知道 iter 方法返回集合中的每一个元素，而 enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回
    // enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用。
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

// fn first_word_new(s: &String) -> &str {
// 因为字符串字面量也是 slice，因此可以
//如果有一个字符串 slice，可以直接传递它。
// 如果有一个 String，则可以传递整个 String 的 slice 或对 String 的引用。
fn first_word_new(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}