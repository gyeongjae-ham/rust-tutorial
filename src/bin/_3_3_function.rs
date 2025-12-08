fn main() {
    println!("Hello, world!");
    another_function();

    // 매개변수
    another_function2(5);

    let x = 5; // 구문식 어떠한 걸 반환하는게 아님 다른 변수에 할당 불가
    // let y = (let x = 5;) 불가능

    // 표현식
    let y = {
        let x = 3;
        x + 1
    };

    // 표현식
    {
        let xx = 3;
        x + 1
    };

    // 표현식의 종결에는 세미콜론을 쓰지 않는다
    // 만약 세미콜론 쓰면 구문으로 변경되어서 반환을 안한다


    five();
}

fn another_function() {
    println!("Another function.");
}

// 매개변수의 타입은 반드시 선언해야 한다
fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

// 반환값을 가지는 함수
fn five() -> i32 {
    // 함수에서도 마지막에 세미콜론 쓰면 구문으로 반환 안한다
    // 사용하는 측에서 변수에 할당하면 에러 뜸(반환값이 없으니까)
    5
}