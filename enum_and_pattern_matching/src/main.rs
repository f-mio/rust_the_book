
fn sec6_1_enum_with_struct() {

    enum IpAddrKind {
        V4,
        V6
    };

    struct IpAddr {
        kind: IpAddrKind,
        address: String
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    println!("list 6-1 is finished!");
}


fn sec6_1_only_enum() {

    // enum IpAddr {
    //     V4(String),
    //     V6(String)
    // }

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("only enum pattern is finished!");
}

fn sec6_1_enum2() {
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String)
    }

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("enum pattern2 is finished!")
}


fn sec6_1_option_test () {
    let x1: Option<i8> = Some(10);
    let x2: Option<i8> = Some(20);

    // println!("Option<t>同士の演算チェック\nx1: Some(10) + x2: Some(20) = {}", x1 + x2);
    // error[E0369]: cannot add `Option<i8>` to `Option<i8>`
    //  --> src/main.rs:62:73
    //  |
    //  62 |     println!("Option<t>同士の演算チェック\nx1: Some(10) + x2: Some(20) = {}", x1 + x2);
    //  |                                                                               -- ^ -- Option<i8>
    //  |                                                                               |
    //  |                                                                               Option<i8>
    //  |
    //  note: the foreign item type `Option<i8>` doesn't implement `Add`
    //  --> /rustc/ed61e7d7e242494fb7057f2657300d9e77bb4fcb/library/core/src/option.rs:594:1
    //  |
    //  = note: not implement `Add`
    println!("Option<t>同士の演算チェック\nx1: Some(10) + x2: Some(20) = {}",
        x1.unwrap_or(0) + x2.unwrap_or(0)
    );
}


fn sec6_2_coin_1() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }

    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

fn sec6_2_match_with_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    println!("sec 6.2 plus_one(5) is {:?}", plus_one(Some(5)));
    println!("sec 6.2 plus_one(None) is {:?}", plus_one(None));
}


fn sec6_2_coin_2() {
}


fn main() {
    sec6_1_enum_with_struct();
    sec6_1_only_enum();
    sec6_1_option_test();
    
    sec6_2_coin_1();
    sec6_2_coin_2();
    sec6_2_match_with_option();
}
