fn main() {
    // 벡터에 아무것도 넣지 않았을 때는 Rust가 어떤 타입의 vector인지 알지 못하므로
    // 제네릭에 표현해 줘야 한다
    let v: Vec<i32> = Vec::new();

    // 초기값으로 벡터 생성
    let v = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();

    // 벡터 값 추가
    // 벡터도 마찬가지로 스코프를 벗어나면 소유권을 잃습니다
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 벡터 요소 읽기
    // []로 불러오는 경우 해당 index가 없을 경우 panic!을 일으킨다
    let third: &i32 = &v[2];
    // 이 문법으로하면 None의 경우 대처할 수 있도록 하는 여지를 준다
    let third: Option<&i32> = v.get(2);

    let mut v = vec![1, 2, 3, 4, 5];

    // 여기서 얻은건 불변 참조자
    let first = &v[0];

    // 해당 부분에서 에러
    // 불변 참조자가 살아있는 상태에서 가변 참조자 쓰려고 하고 있기 때문에
    v.push(6);

    // 벡터 내의 값들 반복처리
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    // enum으로 여러 타입 저장하기
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}