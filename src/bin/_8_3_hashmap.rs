use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // HashMap 제네릭에 _ 쓰는 이유는 다양한 타입으로 변경될 수 있어서
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name과 field_value은 이 지점부터 유효하지 않습니다.
    // i32와 같은 primitive 타입과 Copy 트레잇을 구현한 타입은 해쉬맵 안으로 복사된다

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 특정 키의 값 덮어쓰기
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // 키에 할당된 값이 없을 경우에만 삽입하기
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}