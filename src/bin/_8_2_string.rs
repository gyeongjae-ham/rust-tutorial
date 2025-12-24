fn main() {
    let mut s = String::new();

    // 초기값을 가진 상태일 경우
    // Display 트레잇이 구현된 타입이라면 to_string()으로 String 생성하기
    let data = "initial contents";

    let s = data.to_string();
    let s = "initial contents".to_string();

    // 이렇게도 만들 수 있음
    let hello = String::from("hello");

    // String 추가할 수 있다
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // 소유권을 잃지 않은 상태에서 추가할 수 있음
    s1.push_str(s2);

    println!("s2 is {}", s2);
    println!("s1 is {}", s1);

    let mut s = String::from("lo");
    s.push('l');

    // String 접합
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // 여기서 s1의 소유권은 이동됐고, s2는 안됐다
    let s3 = s1 + &s2;

    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // String 인덱싱
    let s1 = String::from("hello");
    // 안된다
    // let h = s1[0];

    // String은 Vec<u8>을 감싼 것
    // 이 값은 4가 나오고, UTF-8으로 인코딩되면 각 글자들이 1바이트를 차지한다
    let len = String::from("Hola").len();

    // 이건 길이가 12가 아니라 24라고 나온다
    // 각각의 유니코드 스칼라 값이 저장소의 2바이트를 차지하기 때문이다
    // 따라서 인덱스는 유효한 유니코드 스칼라 값과 항상 대응되지 않을 것이다
    let len = String::from("Здравствуйте").len();

    let hello = "Здравствуйте";

    /*
    1. Java/Kotlin과 Rust의 결정적 차이
    Java/Kotlin: 문자열을 내부적으로 UTF-16으로 저장합니다. 대부분의 글자가 2바이트 고정이라서 "몇 번째 글자"를 찾기가 상대적으로 쉽습니다. (물론 이모지 같은 건 여기서도 2글자 취급받는 문제가 있죠.)

    Rust: 문자열을 내부적으로 UTF-8로 저장합니다. UTF-8은 글자마다 1바이트에서 4바이트까지 크기가 제각각인 가변 길이 인코딩입니다.

    2. 왜 s[0]을 금지했나? (성능과 정확성)
    ① "상수 시간(O(1))"의 원칙
    Java의 charAt(i)는 내부 배열의 인덱스로 바로 접근하므로 매우 빠릅니다. 하지만 Rust의 UTF-8에서 "5번째 글자"를 찾으려면, 앞에서부터 읽으며 각 글자가 몇 바이트짜리인지 일일이 계산해야 합니다.

    만약 s[i]를 허용하면, 사용자는 이게 O(1)처럼 빠를 거라고 착각하겠지만 실제로는 O(n)만큼 느려집니다. Rust는 "성능 착각"을 불러오는 문법을 허용하지 않습니다.

    ② "글자"의 정의가 모호함
    보내주신 본문의 힌디어 예시처럼, 우리가 눈으로 보는 글자 1개가 실제로는 여러 개의 유니코드 스칼라 값으로 합쳐진 것일 수 있습니다.

    Bytes (바이트): 컴퓨터가 보는 데이터 [224, 164, ...]

    Scalars (스칼라): Rust의 char 타입. 눈으로 볼 땐 1글자인데 실제론 2개 이상의 스칼라가 합쳐진 경우가 많음.

    Graphemes (문자소): 사람이 눈으로 읽는 진짜 '글자' 단위.

    Rust는 s[0]이라고 했을 때, 바이트 1개를 줄지, 스칼라 1개를 줄지, 문자소 1개를 줄지가 명확하지 않기 때문에 아예 컴파일 단계에서 막아버리는 것입니다.
    let answer = &hello[0];
     */

    // 이건 앞 두 자리를 슬라이싱 성공하겠지만
    let s = &hello[0..4];

    /*
    유효하지 않은 인덱스로 슬라이싱하면 런타임 에러가 발생한다
    let s = &hello[0..1];

    thread 'main' panicked at 'index 0 and/or 1 in `Здравствуйте` do not lie on
    character boundary', ../src/libcore/str/mod.rs:1694
     */

    // 개별적으로 반복하려면 chars 메소드를 이용하는게 제일 좋다
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}