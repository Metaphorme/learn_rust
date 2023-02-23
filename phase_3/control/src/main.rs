fn main() {
    let number = 5;

    if number > 5 {
        println!("number > 5");
    } else if number > 3 {
        println!("3 < number =< 5")
    } else {
        println!("numbrt =< 3")
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number)
}
