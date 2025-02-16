// enu with data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitted"),
        Message::Move { x, y } => println!("Moved to ({x}, {y})"),
        Message::Write(text) => println!("> {}", text),
        Message::ChangeColor(r, g, b) => println!("Changed color to {} {}, {}", r, g, b),
    }
}

enum AnimalType {
    Dog,
    Cat,
}

struct Animal {
    age: u8,
    name: String,
    kind: AnimalType,
}

impl Animal {
    fn new_cat(name: &str) -> Self {
        Self {
            age: 0,
            name: name.to_string(),
            kind: AnimalType::Cat,
        }
    }

    fn new_dog(name: &str) -> Self {
        Self {
            age: 0,
            name: name.to_string(),
            kind: AnimalType::Dog,
        }
    }

    fn change_age(&mut self, age: u8) {
        self.age = age;
    }

    fn speak(&self) {
        match self.kind {
            AnimalType::Dog => println!("{}({}): Woof!", self.name, self.age),
            AnimalType::Cat => println!("{}({}): Meow!", self.name, self.age),
        }
    }
}

fn main() {
    // Structs: a lot of things to group together
    // Enum: a lot of choices, and need to select one

    // unit struct
    struct FileDirectory;

    // tuple struct
    struct Color(u8, u8, u8);

    let my_color = Color(255, 0, 0);
    println!("mycolor: {}, {}, {}", my_color.0, my_color.1, my_color.2);

    struct SizeAndColor {
        size: u32,
        color: Color,
    }

    let size_and_color = SizeAndColor {
        size: 100,
        color: my_color,
    };

    println!(
        "size: {}, color: {}, {}, {}",
        size_and_color.size, size_and_color.color.0, size_and_color.color.1, size_and_color.color.2
    );

    struct Country {
        name: String,
        population: u32,
    }

    let country = Country {
        name: String::from("USA"),
        population: 331_002_651,
    };

    println!(
        "Country: {}, Population: {}",
        country.name, country.population
    );

    // enum
    enum Climate {
        Hot,
        Cold,
        Warm,
    }

    let climate = Climate::Warm;

    match climate {
        Climate::Hot => println!("It's hot"),
        Climate::Cold => println!("It's cold"),
        Climate::Warm => println!("It's warm"),
    }

    let q = Message::Quit;
    let m = Message::Move { x: 3, y: 4 };
    let w = Message::Write(String::from("Hello"));
    let c = Message::ChangeColor(255, 0, 0);

    process_message(q);
    process_message(m);
    process_message(w);
    process_message(c);

    let mut maru = Animal::new_dog("Maru");
    let mut jiji = Animal::new_cat("Jiji");

    maru.change_age(18);
    jiji.change_age(10);

    maru.speak();
    jiji.speak();
}
