use std::collections::{HashMap, BTreeMap};

struct City {
    name: String,
    population: HashMap<i32, i32>,
}

struct City2 {
    name: String,
    population: BTreeMap<i32, i32>,
}


fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };
    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);

    // Esse for pode mudar todas vez q eh rodado pois HashMap nao possui ordem
    for (year, population) in tallinn.population {
        println!("In {year}, Tallin had a population of {population}")
    }

    let mut tallinn2 = City2 {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };
    tallinn2.population.insert(2020, 437_619);
    tallinn2.population.insert(1372, 3_250);
    tallinn2.population.insert(1851, 24_000);

    // BTreeMap possui ordem
    println!("Usando BTreeMap");
    for (year, population) in tallinn2.population {
        println!("In {year}, Tallin had a population of {population}")
    }

    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");
    println!("{:?}", book_hashmap.get(&1));

    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    let key = 1;
    match book_hashmap.get(&key) {
        Some(val) => println!("Key {key} has a value already: {val}"),
        None => {
            book_hashmap.insert(key, "Le Petit Prince");
        }
    }
    println!("{:?}", book_hashmap.get(&1));
}