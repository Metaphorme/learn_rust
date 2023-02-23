use std::io;

fn main() {
    println!("Please enter Fahrenheit:");

    let mut fah = String::new();

    io::stdin()
        .read_line(&mut fah)
        .expect("Failed to readline");
    
    let fah: f64 = fah.trim().parse().expect("Please type a number!");
    
    let fah = 5.0 / 9.0 * (fah - 32.0);

    println!("Celsius is {}", fah)
}
