use std::time::Instant;

fn main() {
    let start = Instant::now();

    let x: i32 = 13;
    println!("Hello, world!");
    println!("{}", x);

    let duration = start.elapsed();
    println!("{:?}", duration);
}
