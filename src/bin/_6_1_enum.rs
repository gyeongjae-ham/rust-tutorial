use std::net::Ipv4Addr;

fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    // 구조체를 이용해서 연관값을 지정하는 경우
    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };

    // enum 자체를 이용하는 방법
    let home = IpAddKindWithValue::V4(String::from("127.0.0.1"));
    let loopback = IpAddKindWithValue::V6(String::from("::1"));

    // enum을 사용하면 구조체보다 열거 타입에 따라 다른 표현을 하기 쉽다
    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));

    // enum도 구현체 정의 가능
    let msg = Message::Write(String::from("hello"));
    msg.call();
}

enum IpAddKindWithValue {
    V4(String),
    V6(String),
}

enum IpAddKind {
    V4,
    V6,
}

// enum을 사용하면 구조체보다 열거 타입에 따라 다른 표현을 하기 쉽다
enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

// 열거형도 impl 정의할 수 있다
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called : {:#?}", self);
    }
}