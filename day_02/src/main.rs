use std::io;

fn uppercase_string_converter(text: &str) -> String {
    let uppercased = text.to_uppercase();
    uppercased
}

fn show_borrowed_string(text: &str) {
    println!("printed from `show_borrowed_string`: {text}")
}

/**
 * This function gives ownership of the string to the function,
 */
fn print_string(text: String) {
    println!("printed from `print_string`: {text}")
}

/**
 * This function takes a reference to the string,
 * so it doesn't own the string, but it can still use it.
 * It doesn't cause any ownership transfer.
 */
fn print_string_with_ref(text: &String) {
    println!("printed from `print_string_with_ref`: {text}");
}

fn main() {
    println!("input 5 characters or more:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let trimmed_input = input.trim();

    let uppercased_input = uppercase_string_converter(trimmed_input);
    let uppercased_slice = &uppercased_input[0..5];

    println!("Uppercased string: {uppercased_slice}");
    println!("Uppercased string (first 5 characters): {uppercased_slice}");

    show_borrowed_string(uppercased_slice);

    let name = String::from("AOI");
    // `country` is moved to function, so it cannot be used again
    print_string(name);
    // This will cause a borrowed value is moved error
    // print_string(country);

    let name = String::from("KANADE");
    print_string_with_ref(&name);
    print_string_with_ref(&name);
    print_string_with_ref(&name);
}
