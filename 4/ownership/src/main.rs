#![allow(unused)]
fn main() {
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}