fn main() {
    // 슬라이스(slice)는 collection을 통째로 참조하는 게 아닌
    // 일부의 연속된 요소를 참조하도록 해준다
    // 슬라이스는 참조자의 일종으로서 소유권을 갖지 않습니다
    let mut s = String::from("hello world");

    let _word = first_word(&s);

    s.clear();
    // 여기서 clear하면 word라는 변수의 5라는 값은 유의미한 값을 지니지 않는다
    // 두번째 단어를 찾는다면 또 관리해야 하는 index만 늘어나므로 불편하다
    // 그래서 슬라이스를 쓰는 것

    s.push_str("hello world");

    // start index는 포함하고
    // end index는 포함하지 않는다 -1 까지
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    // range 표현할 때 인덱스 0부터 시작하는 경우에는 앞의 값을 생략할 수 있다

    let s = String::from("hello world");

    // 두 표현이 같다
    let slice = &[0..2];
    let slice = &[..2];

    // 맨 마지막까지 포함하는 경우 생략할 수 있다
    let len = s.len();

    // 두 표현이 같다
    let slice = &s[3..len];
    let slice = &s[3..];

    // 앞 뒤를 다 생략하면 전체 문자열이 슬라이스로 생성된다
    let slice = &s[0..len];
    let slice = &s[..];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}