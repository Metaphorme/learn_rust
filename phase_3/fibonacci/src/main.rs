use std::io;
use std::time::Instant;

fn fib(n: i64) -> i64 {
    if n == 1 || n == 2 {
        1
    } else {
        fib(n - 2) + fib(n - 1)
    }
}

fn main() {
    println!("Please enter n:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n :i64 = n.trim().parse().expect("Please type a number!");
    
    let start_time = Instant::now();

    println!("F{} = {}", n, fib(n));

    let duration_time = start_time.elapsed();

    println!("In {:?}", duration_time);
}
