fn main() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;
    // 참조하고 있는 메모리 주소를 넘긴 상태라면 스코프를 벗어난 시점에서 drop 함수가 두 번 호출되면서
    // 중복 해제 요청을 할 수 있게 된다

    // println!("{}, world!", s1); 따라서 이 시점에서 s1을 사용할 수 없도록 한다 소유권이 이전됐으므로
    // 다른 언어에서는 얕은 복사, 깊은 복사 개념이 존재하지만, Rust의 경우 이동이라고 표현한다

    let s3 = s2.clone(); // 소유권 이전이 아닌 깊은 복사로 새로운 힙에 할당하므로
    println!("s3 = {}, s2 = {}", s3, s2); // 여기서 둘 다 쓸 수 있다

    // 여기서 보면 clone 하지 않아도 둘 다 쓸 수 있는데
    // 이유는 정수형 등 primitive 타입이기 때문에 스택에 고정되기 때문입니다
    // 이렇게 Copy가 가능한 타입들
    // 모든 정수형 타입
    // true, false
    // 모든 부동 소수점 타입
    // 문자 타입
    // primitive 타입으로 구성된 튜플
    // ex. i32, i32는 가능하지만 / i32, String은 불가
    println!("x = {}, y = {}", x, y);
    divider();

    owner_function();

    return_function();
}

fn owner_function() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    divider();
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
    divider();
}

fn divider() {
    println!("===========================");
}

fn return_function() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    // s2 s3로 이동
    // s3로 함수 반환값 이동
    let s3 = takes_and_gives_back(s2);
    let (s4, len) = calculate_length(s3); // 튜플 반환값 받기

    println!("The length of '{}' is {}.", s4, len);
    divider();
}
// s1은 이 스코프 벗어나면서 Drop

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
    // Drop
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length) // 튜플로 반환
}