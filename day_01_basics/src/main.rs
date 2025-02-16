fn give_number() -> u32 {
    // no semicolon is needed here
    // it returns 8
    8
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    // Integers
    let num = 42;
    println!("The value of num is: {}", num);

    // Chars

    let first_letter = 'a';
    println!("The first letter of the alphabet is: {}", first_letter);

    let space = ' ';
    println!(
        "The ASCII value of the space character is: {}",
        space as u32
    );

    let other_language_char = '§';
    println!("The Unicode character: {}", other_language_char);

    let cat_face = '😺';
    println!("The cat face emoji is: {}", cat_face);

    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());
    println!("Size of 𓅱: {}", "𓅱".len());

    let str1 = "Hello!";
    println!(
        "str1 is {} bytes and also {} characters",
        str1.len(),
        str1.chars().count()
    );
    let str2 = "안녕!";
    println!(
        "str2 is {} bytes and also {} characters",
        str2.len(),
        str2.chars().count()
    );

    println!("{:?}", "a".as_bytes());
    println!("{:?}", "ß".as_bytes());
    println!("{:?}", "国".as_bytes());
    println!("{:?}", "𓅱".as_bytes());

    // Floats
    let float_num = 5.0;
    println!("The value of float_num is: {}", float_num);

    let float_64: f64 = 5.0;
    let float_32: f32 = 5.0;
    // println!("{} + {} = {}", float_64, float32, float_64 + float32);
    println!(
        "{} + {} = {}",
        float_64,
        float_32,
        float_64 + float_32 as f64
    );

    // both infered as f64
    let float_64 = 3.0;
    let float_32 = 3.0;
    println!("{} + {} = {}", float_64, float_32, float_64 + float_32);

    // both infered as f32
    let float_64: f32 = 3.0;
    let float_32 = 3.0;
    println!("{} + {} = {}", float_64, float_32, float_64 + float_32);

    println!("Hello, world number {}", give_number());

    println!("Result of 2 * 3 = {}", multiply(2, 3));

    let a = 3;
    let b = 4;
    println!("{a} * {b}");
    println!("{a} * {b} = {}", multiply(a, b));

    // Display, Debug
    let doesnt_print = ();
    println!("Doesn't print: {:?}", doesnt_print);

    // Mutable
    let mut my_number = 5;
    my_number = 1;
    println!("The value of my_number is: {}", my_number);

    // Shadowing
    let brian = true;
    println!("Brian?, {brian}");
    {
        let brian = "I'm Brian";
        println!("Brian?, {brian}");
    }
    println!("Brian?, {brian}");

    let final_number = {
        let x = 3;
        let y = 5;
        let x = x + y;
        x + 2
    };
    println!("Final number: {final_number}");
}
