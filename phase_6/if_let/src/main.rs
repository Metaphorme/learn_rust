enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    let some_u8_value = Some(0u8);

    // match
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if-let
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = Coin::Penny;
    let mut count = 0;

    // match
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // if-let
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
