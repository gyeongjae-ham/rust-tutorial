fn main() {
    // 변경가능한 문자열 타입
    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{}", s);
}
