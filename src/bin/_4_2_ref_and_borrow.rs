fn main() {
    let s1 = String::from("hello");

    // 소유권을 가져오지 않고 참조값만 넘긴다
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    did_not_change(&s1);
    divide();

    let mut s2 = String::from("hello2");

    // 가변 참조자는 한 변수에 하나밖에 쓸 수 없다
    // 불변 참조자가 있는 상태에서도 가변 참조자를 만들 수 없다 반대의 경우도 마찬가지다
    // 가변 참조자가 하나밖에 없기 때문에 제어가 원활하다
    // race condition이 없다
    change(&mut s2);
    println!("s2 is {}", s2);
    divide();

    immutable_and_mutable();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn did_not_change(_some_string: &String) {
    // some_string.push_str(", world");
    // 소유권을 가진 상태가 아닌 빌린 상태라면
    // 해당 값을 수정할 수 없다
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn immutable_and_mutable() {
    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    println!("{} and {}", r1, r2);
    // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

    let r3 = &mut s; // 문제없음

    println!("{}", r3);
    divide();
}

fn divide() {
    println!("==================")
}