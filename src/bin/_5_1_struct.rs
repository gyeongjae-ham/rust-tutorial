struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 가변성은 인스턴스 전체가 가지게 된다
    // 특정 필드만 가변으로 만들 수 없다...ㅜㅜ 이건 너무 슬픈데?
    let mut user1 = User {
        active: true,
        // 구조체 문자열 정의를 &str 문자열 슬라이스 대신에
        // 소유권을 가지도록 String으로 정의한 이유
        // 구조체 인스턴스가 유효한 동안 각 인스턴스 내의 모든 데이터가 유효하도록 하기 위해서
        // 소유권 잃으면 인스턴스가 동작하지 않으므로 ㅇㅇ 이건 안정성 지리긴 한다
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // 일부만 업데이트할 때 이렇게 하면 불편
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // js처럼 가능
    // 다른 값들과 마찬가지로 user2를 생성한 후에는 user1 사용 불가능
    // username이 이동했기 때문 primitive 필드들만 복사했다면 계속 사용 가능
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let user3 = User {
        email: String::from("primitive@test.com"),
        username: String::from("primitive"),
        ..user2
    };

    // user3에서 primitive field만 복사해서 user2 사용가능 소유권 이동 안했으니까
    let test = user2.active;

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    // 매개변수명이랑 구조체 필드명이 같다면 생략가능
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// 튜플 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 필드가 없는 유사 유닛 구조체
struct AlwaysEqual;