fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; 에러 발생! 불변
    println!("The value of x is: {x}");

    // mut를 붙이면 가변
    let mut y = 7;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    // 상수
    // 상수는 mut와 함께 쓸 수 없다, 상수는 항상 불변
    // 상수는 선언된 스코프 내에서 프로그램이 동작하는 전체 시간 동안 유효
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of const is: {THREE_HOURS_IN_SECONDS}");

    // 섀도잉
    // 새 변수를 이전 변수명과 같은 이름으로 선언하는 것
    let xx = 5;
    let xx = xx + 1;

    {
        let xx = xx + 2;
        println!("The value of x in the inner scope is: {xx}");
    }

    println!("The value of x in the outer scope is: {xx}");

    // 같은 변수명으로 새로운 변수를 만드는 것이기 때문에 다른 타입도 가능
    let spaces = "  ";
    let spaces = spaces.len();
}