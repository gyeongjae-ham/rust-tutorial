pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    // test 환경이기 때문에 같은 루트에 있어도 use로 사용해 줘야 한다
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
