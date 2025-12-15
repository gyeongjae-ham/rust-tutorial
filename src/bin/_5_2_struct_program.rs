fn main() {
    let rect1 = (50, 30);
    let rect2 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
    println!(
        "The area_struct of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    // primitive type의 경우 기본적으로 Display 포맷팅이 적용된 상태로 출력이 잘되지만
    // 구조체의 경우 print 방법도 다르며 어노테이션도 붙여줘야 한다
    println!("rect2 is {:?}", rect2);
    // :#?로 출력하면 포맷팅돼서 출력된다
    println!("rect2 is {:#?}", rect2);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
