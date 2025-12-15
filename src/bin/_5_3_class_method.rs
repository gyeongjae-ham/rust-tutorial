#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    // 연관 함수
    // 첫 번째 파라미터로 &self를 가지지 않는 메소드
    // 호출 시에 ::로 호출해야 한다
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    let rect = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}
