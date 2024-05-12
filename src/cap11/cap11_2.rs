use std::rc::Rc;

// Exemplo 1
#[derive(Debug)]
struct City {
    name: Rc<String>,
    population: u32,
    city_history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>,
}

// Exemplo 2

fn take_a_string(input: Rc<String>) {
    println!("It is: {input}")
}

// Exemplo 3 -  Lifetime

// Exemplo de codigo com Lifetime
// #[derive(Debug)]
// struct City<'a> {
//     name: &'a str,
//     date_founded: u32,
// }
// #[derive(Debug)]
// struct Country<'a> {
//     cities: Vec<City<'a>>,
// }
// #[derive(Debug)]
// struct World<'a> {
//     countries: Vec<Country<'a>>,
// }

#[derive(Debug)]
struct City2 {
    name: Rc<String>,
    date_founded: u32,
}
#[derive(Debug)]
struct Country2 {
    cities: Vec<City2>,
}
#[derive(Debug)]
struct World2 {
    countries: Vec<Country2>,
}

fn main() {
    println!("Exemplo 1");

    let calgary_name = Rc::new("Calgary".to_string());
    let calgary_history =
        Rc::new("Calgary began as a fort called Fort ➥Calgary that...".to_string());
    let random_rc = Rc::new(1);

    let calgary = City {
        name: Rc::clone(&calgary_name),
        population: 1_200_000,
        city_history: Rc::clone(&calgary_history),
    };

    let canada_cities = CityData {
        names: vec![Rc::clone(&calgary_name)],
        histories: vec![Rc::clone(&calgary_history)],
    };

    println!("Calgary history is: {}", calgary.city_history);
    println!(
        "Temos {} owner para calgary.city_history",
        Rc::strong_count(&calgary.city_history)
    );
    println!(
        "Temos {} owner para calgary_history",
        Rc::strong_count(&calgary_history)
    );
    println!(
        "Temos {} owner para random_rc",
        Rc::strong_count(&random_rc)
    );

    println!("Exemplo 2");

    let user_name = Rc::new(String::from("User MacUserson"));

    take_a_string(Rc::clone(&user_name));
    take_a_string(Rc::clone(&user_name));

    println!("Exemplo 3");

    // Exemplo com Lifetime
    // let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    // let my_city = City {
    //     name: &city_names[0],
    //     date_founded: 1921,
    // };
    // println!("{} was founded in {}", my_city.name, my_city.date_founded);

    let city_names = vec![
        Rc::new("Ichinomiya".to_string()),
        Rc::new("Kurume".to_string()),
    ];
    let my_city = City2 {
        name: Rc::clone(&city_names[0]),
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    // Como criar City2, Country2 e Wordl2 sem sofre move???
}
