extern crate communicator;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules()
        }
    }
}

// use를 이용해서 모듈을 가져옴으로써 긴 함수 호출을 막아준다
// 이렇게 명시하면 of 모듈까지만 불러온다
use a::series::of;

// 이렇게 하면 함수를 가져온 것
// use a::series::of::nested_modules;
// 사용 시에 nested_modules(); 이렇게 사용할 수 있다

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{ Red, Yellow };

enum TrafficLight2 {
    Red,
    Yellow,
    Green,
}

// asterisk로 전체를 가져올 수 있다
use TrafficLight2::*;

fn main() {
    communicator::client::connect();

    // 경로를 지정할 경우 너무 길어질 수 있다
    a::series::of::nested_modules();

    of::nested_modules();

    // enum도 use로 가져올 수 있다
    let red = Red;
    let yellow = Yellow;

    // enum 값을 asterisk로 전체 가져올 수 있다
    let green = Green;
}