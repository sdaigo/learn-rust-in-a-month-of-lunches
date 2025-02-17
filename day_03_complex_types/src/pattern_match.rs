use std::fmt::Display;

/**
 * This function just prints provided option value
 * Therefore not need a ownership of it, accept argument as a reference.
 */
fn some_or_none<T: Display>(option: &Option<T>) {
    match option {
        Some(value) => println!("is some! Value: {value}"),
        None => println!("is none! :("),
    }
}

fn what_type_of_integer_is_this(value: i32) {
    match value {
        1 => println!("The number one"),
        2 | 3 => println!("The number two or three"),
        4..=10 => println!("The number is between 4 and 10"),
        _ => println!("Some other kind of number"),
    }
}

/**
 * Pattern matching with destructuring
 */
fn destructuring_tuple(t: &(i32, i32, i32)) {
    match t {
        (first, ..) => println!("First element: {first}"),
    }

    match t {
        (.., last) => println!("Last element: {last}"),
    }

    match t {
        (_, middle, _) => println!("Middle element: {middle}"),
    }

    match t {
        (first, middle, last) => println!("First: {first}, Middle: {middle}, Last: {last}"),
    }
}

fn match_with_guards(value: i32, choose_first: bool) {
    match value {
        v if v == 1 && choose_first => println!("First match: 1"),
        v if v == 1 && !choose_first => println!("Second match: 1"),
        v if choose_first => println!("First match: {v}"),
        v if !choose_first => println!("Second match: {v}"),
        _ => println!("Fell through to the default case"),
    }
}

enum DistinctType {
    Name(String),
    Count(i32),
}

fn match_enum_types(enum_types: &DistinctType) {
    match enum_types {
        DistinctType::Name(name) => println!("Name: {name}"),
        DistinctType::Count(count) => println!("Count: {count}"),
    }
}

enum CatColor {
    Black,
    White,
    Brown,
    Blue,
    Grey,
}

struct Cat {
    name: String,
    color: CatColor,
}

fn match_on_black_cats(cat: &Cat) {
    match cat {
        Cat {
            name,
            color: CatColor::Black,
        } => println!("{name} is black"),
        Cat { name, color: _ } => println!("{name} is not black"),
    }
}

pub fn main() {
    some_or_none(&Option::Some(5));
    some_or_none::<i32>(&Option::None);

    what_type_of_integer_is_this(1);
    what_type_of_integer_is_this(2);
    what_type_of_integer_is_this(3);
    what_type_of_integer_is_this(4);
    what_type_of_integer_is_this(5);
    what_type_of_integer_is_this(10);
    what_type_of_integer_is_this(11);

    destructuring_tuple(&(1, 2, 3));

    match_with_guards(1, true); // first match: 1
    match_with_guards(1, false); // second match: 1
    match_with_guards(2, true); // first match: 2
    match_with_guards(2, false); // second match: 2

    match_enum_types(&DistinctType::Name("AOI".to_string()));
    match_enum_types(&DistinctType::Count(100));

    match_on_black_cats(&Cat {
        name: "Jiji".to_string(),
        color: CatColor::Black,
    });
    match_on_black_cats(&Cat {
        name: "Tiro".to_string(),
        color: CatColor::White,
    });
}
