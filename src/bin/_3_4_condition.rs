fn main() {
    let number = 3;

    // if 문
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let boolean = true;

    // boolean true, false로 태우기
    if boolean {
        println!("condition was true");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let 구문에서 if문 쓰기
    let condition = true;
    // if 문의 타입과 else 문의 타입이 일치해야만 한다
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}