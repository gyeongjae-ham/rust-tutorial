fn main() {
    // 정적 타입 언어이므로 타입 지정해야 한다
    let guess: u32 = "42".parse().expect("Not a number!");

    // rust 정수형
    // 부호 있음: i8, i16, i32, i64, i128, isize
    // 부호 없음: u8, u16, u32, u64, u128, usize
    // 2의 보수형으로 2의 n승 - 1이 범위
    // 일반적으로 i32가 좋은 시작 지점
    // isize, usize는 컬렉션 종류의 인덱스로 활용

    // 부동 소수점 타입
    // f32, f64
    let x = 2.0; // f64가 기본 타입
    let y: f32 = 3.0; // f32

    // 수치 연산
    // 덧셈
    let sum = 5 + 10;

    // 뺄셈
    let difference = 95.5 - 4.3;

    // 곱셈
    let product = 4 * 30;

    // 나눗셈
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 결괏값은 -1입니다

    // 나머지 연산
    let remainder = 43 % 5;


    // boolean 타입
    let t = true;
    let f: bool = false; // 명시적인 타입 어노테이션

    // 문자 타입
    // char가 기본 알파벳 타입
    // literal은 큰 따옴표, char는 작은 따옴표
    let c = 'z';
    let z: char = 'ℤ'; // 명시적인 타입 어노테이션
    let heart_eyed_cat = '😻';

    // 튜플
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 배열
    // 길이를 알고 있다면 고정적인 배열 아니라면 가변적인 vector를
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
}