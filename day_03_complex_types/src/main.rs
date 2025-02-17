mod clean_match;
mod pattern_match;

fn match_colors(color: (i32, i32, i32)) {
    match color {
        (r, _, _) if r < 10 => println!("not much red"),
        (_, g, _) if g < 10 => println!("not much green"),
        (_, _, b) if b < 10 => println!("not much blue"),
        _ => println!("each color has at least 10"),
    }
}

fn main() {
    // Array
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = ["one", "two", "three", "four", "five"];

    let arr3 = ["a"; 5]; // ["a", "a", "a", "a", "a"]

    println!("array 1: {:?}", arr1);
    println!("3rd on array 1: {}", arr1[2]);

    let two_to_five = &arr2[1..5];
    let start_at_one = &arr2[1..];
    let end_at_three = &arr2[..3];
    let all_elements = &arr2[..];

    println!("two to five: {two_to_five:?}");
    println!("start at one: {start_at_one:?}");
    println!("end at three: {end_at_three:?}");
    println!("all elements: {all_elements:?}");

    for a in arr3.iter() {
        print!("element: {a} ");
    }
    println!();

    // Vector
    let mut vec = Vec::new();
    println!("vector: {vec:?}");

    vec.push(1);
    vec.push(2);

    println!("vector: {vec:?}");

    let mut colors = vec!["red", "blue", "green"];
    colors.push("yellow");

    for color in colors.iter() {
        println!("{color}");
    }

    // Tuple
    let random_tuple = (1, "two", 3.0);
    println!("{}, {}, {}", random_tuple.0, random_tuple.1, random_tuple.2);

    let (a, b, c) = random_tuple;
    println!("{a}, {b}, {c}");

    // Control flow: if
    let my_number = 5;

    if my_number % 2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }

    // if expression
    let result = if my_number % 3 == 0 {
        "The number is divisible by 3"
    } else {
        "The number is not divisible by 3"
    };

    println!("Result: {result}");

    // pattern matching: match
    match my_number {
        0 => println!("The number is zero"),
        5 => println!("The number is 5"),
        _ => println!("The number is not 0, 1, 2, 3 or 4"),
    }

    let result = match my_number {
        0 => "The number is zero",
        5 => "The number is 5",
        _ => "The number is not 0, 1, 2, 3 or 4",
    };

    println!("Result: {result}");

    let red = (200, 0, 0);
    let green = (0, 200, 0);
    let blue = (100, 50, 200);
    let gray = (50, 50, 50);

    match_colors(red);
    match_colors(green);
    match_colors(blue);
    match_colors(gray);

    // loop
    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter = {counter}");

        if counter == 5 {
            break;
        }
    }

    for i in 1..=5 {
        println!("i = {i}");
    }

    println!("\n----------------\n");

    pattern_match::main();

    println!("\n----------------\n");

    clean_match::main();
}
