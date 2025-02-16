fn see_if_number_is_even(number: i32) -> Result<(), ()> {
    if number % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn check_if_five(number: i32) -> Result<String, String> {
    if number == 5 {
        Ok("The number is 5.".to_string())
    } else {
        Err(format!("The number is not 5. Expected 5 Got: {number}"))
    }
}

pub fn main() {
    if see_if_number_is_even(5).is_ok() {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }

    for n in 4..=7 {
        println!("Checking if {} is five: {:?}", n, check_if_five(n));
    }
}
