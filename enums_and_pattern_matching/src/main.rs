fn main() {
    println!("Hello, world!");

//  <T> is a generic that means that the Some variant 
// of the Option enum can hold one piece of data of any type, 
// and that each concrete type that gets used in place of T 
// makes the overall Option<T> type a different type

#[derive(Debug)] 
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

let five = Some(5);
println!("A coin is {} cents", value_in_cents(Coin::Nickel));
println!("Sum is {:?}", add_one_if_number(five));

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 1,
        Coin::Nickel => 5,
        Coin::Penny => 10,
        Coin::Quarter (state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn add_one_if_number (x: Option<i32>) -> Option<i32> {
 match  x {
    None => None,
    Some(i) => Some(i + 1)
 }
}
// Instead of adding boilerplate code such as this
// to satify match exhaustiveness
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}"),
    _ => (),
}

// make use of if let instead
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
}
