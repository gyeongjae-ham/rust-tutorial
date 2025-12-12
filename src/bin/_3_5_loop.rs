fn main() {
    // 무한 반복
    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
    divider();

    looper();

    while_looper();

    while_collection_looper();

    for_collection_looper();

    for_looper_with_range();
}

fn divider() {
    println!("=========================")
}

fn looper() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
    divider();
}

fn while_looper() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("Liftoff!");
    divider();
}

fn while_collection_looper() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    divider();
}

fn for_collection_looper() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    divider();
}

fn for_looper_with_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("Liftoff!");
    divider();
}