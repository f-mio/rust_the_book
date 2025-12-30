

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let penny_coin = Coin::Penny;
    println!("This coin value is {} cents.",value_in_cents(penny_coin));
    println!("This coin value is {} cents.",value_in_cents(Coin::Nickel));
    println!("This coin value is {} cents.",value_in_cents(Coin::Dime));
    println!("This coin value is {} cents.",value_in_cents(Coin::Quarter));
}
