pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        use crate::outermost;

        pub fn middle_function() {
            // 자식은 부모의 private에 접근 가능
            super::middle_secret_function();
        }

        fn secret_function() {}
    }
}

fn try_me_faile() {
    outermost::middle_function();

    // outermost module이 같은 루트에 있어서 middle_function은 접근 가능
    // 하지만 middle_secret_function의 상위 모듈은 outermost이고 이 외부로 개방되지 않아서 접근 불가능
    // outermost::middle_secret_function();
    // 마찬가지 이유로 inside module에 접근 불가능
    // outermost::inside::middle_function();
    // outermost::inside::secret_function();
}

mod outermost_success {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    pub mod inside {
        use crate::outermost_success;

        pub fn middle_function() {}

        pub fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();

    outermost_success::middle_secret_function();
    outermost_success::inside::middle_function();
    outermost_success::inside::secret_function();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
