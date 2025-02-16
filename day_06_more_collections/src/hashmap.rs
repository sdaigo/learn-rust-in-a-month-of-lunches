use std::collections::{BTreeMap, HashMap};

type Year = i32;
type Population = i32;

struct City {
    name: String,
    population: HashMap<Year, Population>,
}

struct CityWithOrder {
    name: String,
    population: BTreeMap<Year, Population>,
}

pub fn main() {
    let mut tallin = City {
        name: String::from("Tallinn"),
        population: HashMap::new(),
    };

    tallin.population.insert(2020, 437_619);
    tallin.population.insert(1372, 3_250);
    tallin.population.insert(1851, 24_000);

    println!("Population in {}:", tallin.name);
    for (year, population) in tallin.population {
        println!("{year}: {population} people");
    }

    let mut tallinn_with_order = CityWithOrder {
        name: String::from("Tallinn"),
        population: BTreeMap::new(),
    };

    tallinn_with_order.population.insert(1372, 3_250);
    tallinn_with_order.population.insert(1851, 24_000);
    tallinn_with_order.population.insert(2020, 437_619);

    println!("Population in {}:", tallinn_with_order.name);
    for (year, population) in tallinn_with_order.population {
        println!("{year}: {population} people");
    }

    let canadian_cities = vec!["Calgary", "Ottawa", "Vancouver", "Winnipeg"];
    let german_cities = vec!["Berlin", "Hamburg", "Munich", "Stuttgart"];

    let mut city_map = HashMap::new();

    for city in canadian_cities {
        city_map.insert(city, "Canada");
    }

    for city in german_cities {
        city_map.insert(city, "Germany");
    }

    println!("{}", city_map["Calgary"]);
    println!("{:?}", city_map.get("Berlin"));
    println!("{}", city_map.get("Berlin").unwrap());
    println!("{:?}", city_map.get("somewhere nice"));
    println!(
        "{}",
        city_map.get("somewhere nice").map_or("unknown", |c| c)
    );

    let book_collection = vec![
        "The Great Gatsby",
        "To Kill a Mockingbird",
        "1984",
        "1984",
        "1984",
    ];

    let mut book_map = HashMap::new();

    for book in book_collection {
        let count = book_map.entry(book).or_insert(0);
        *count += 1;
    }

    for (book, count) in book_map {
        println!("Do we have {book}? {count}");
    }
}
