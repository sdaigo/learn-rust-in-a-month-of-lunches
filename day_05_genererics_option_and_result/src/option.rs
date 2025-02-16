use std::fmt::Debug;

fn take_fifth_item<T: Copy>(value: Vec<T>) -> Option<T> {
    if value.len() >= 5 {
        Some(value[4])
    } else {
        None
    }
}

fn print<T: Debug>(index: Option<T>) {
    match index {
        Some(index) => println!("The fifth item is {index:?}"),
        None => println!("Not Found..."),
    }
}

pub fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let index = take_fifth_item(v);

    print(index);

    // or

    if index.is_some() {
        println!("The fifth item is {}", index.unwrap());
    } else {
        println!("Not Found...");
    }

    let v = vec![1, 2, 3];
    let index = take_fifth_item(v);

    print(index);
}
