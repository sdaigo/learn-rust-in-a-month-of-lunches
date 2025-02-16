use std::fmt::{Debug, Display};

fn return_item<T>(item: T) -> T {
    println!("Here is your item");
    item
}

// Tells the compiler that T must implement Debug trait to use this function
fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}");
}

fn compare_and_display<T, U>(statement: T, input_1: U, input_2: U)
where
    T: Display,
    U: Display + PartialOrd,
{
    println!(
        "{statement}! Is {input_1} greater than {input_2}? {}",
        input_1 > input_2
    );
}

#[derive(Debug)]
struct Point;

pub fn main() {
    let item = return_item(42);
    println!("{:?}", item);

    print_item(item);

    let point = Point;
    print_item(point);

    compare_and_display("Listen up", 8, 6);
}
