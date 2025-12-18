enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum CoinWithState {
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

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("================================");

    // enum의 variant가 다른 enum을 들고 있는 경우
    println!("{}", value_in_cents_with_state(CoinWithState::Quarter(UsState::Alaska)));
    println!("================================");

    // Option<T>를 이용하는 매칭
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?}, {:?}", five, six, none);
}

fn value_in_cents_with_state(coin: CoinWithState) -> u32 {
    match coin {
        CoinWithState::Penny => {
            println!("Lucky penny!");
            1
        },
        CoinWithState::Nickel => 5,
        CoinWithState::Dime => 10,
        CoinWithState::Quarter(state) => {
            println!("Lucky quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}