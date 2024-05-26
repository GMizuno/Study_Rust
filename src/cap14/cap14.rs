// Exemplo 1
mod print_thigns {
    use std::fmt::Display;

    pub fn prints_one_thing<T: Display>(input: T) {
        println!("{}", input);
    }
}

// Exemplo 2
mod country {
    fn print_country(country: &str) {
        println!("We are in the country of {country}");
    }
    pub mod province {
        fn print_province(province: &str) {
            println!("in the province of {province}");
        }
        pub mod city {
            pub fn print_city(country: &str, province: &str, city: &str) {
                crate::country::print_country(country);
                super::super::print_country(country);
                crate::country::province::print_province(province);
                super::print_province(province);
                println!("in the city of {city}");
            }
        }
    }
}

fn main() {
    // Exemplo 1
    use print_thigns::prints_one_thing;

    prints_one_thing(6);
    prints_one_thing("Trying");

    // Exemplo 2
    country::province::city::print_city("Canada", "New Brunswick", "Moncton");
}
