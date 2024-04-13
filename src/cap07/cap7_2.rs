use std::process::ChildStdin;

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}.", city.name, city.population)
        }
    }
}

fn main() {
    let array_vec = Vec::from([8, 9, 10]);
    println!("Vec from array: {array_vec:?}");

    let str_vec = Vec::from("What kinf of Vec am I?");
    println!("Vec from str: {str_vec:?}");

    let string_vec = Vec::from("What will a String be?".to_string());
    println!("Vec from String: {string_vec:?}");

    println!("\n\nCITIES\n\n");

    let helsinki = City::new("Helsinki", 632_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    let finland = Country::from(finland_cities);

    finland.print_cities()
}
