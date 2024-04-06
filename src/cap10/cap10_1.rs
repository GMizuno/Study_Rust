#[derive(Debug)]
struct City {
    name: &'static str, // Se fosse &str seria um problema, pois se o valor da referencia for limpando o q seria feito com esse referencia?
    date_founded: u32,
}

#[derive(Debug)]
struct City2<'a> {
    name: &'a str, // Se fosse &str seria um problema, pois se o valor da referencia for limpando o q seria feito com esse referencia?
    date_founded: u32,
}

struct Adventurer<'a> {
    name: &'a str,
    hit_point: u32,
}

impl Adventurer<'_> {
    // Se nao incluir <'_> esse anonymous lifetime o compilador retornar um erro
    fn take_damage(&mut self) {
        self.hit_point -= 20;
        println!("{} has {} hit point left!!!", self.name, self.hit_point);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit point left!!!", self.name, self.hit_point)
    }
}

fn main() {
    let city = City {
        name: "Niteroi",
        date_founded: 1995,
    };
    let my_city = city;
    println!("{my_city:?}");

    // Retorna um erro, pois city_name não "vivera" tempo suficiente
    // let city_name = vec!["Niteroi".to_string(), "Rio de Janeiro".to_string()];
    // let my_city2 = City {
    //     name: &city_name[0],
    //     date_founded: 1995,
    // };
    // println!("{my_city2:?}");

    let city_name = vec!["Niteroi".to_string(), "Rio de Janeiro".to_string()];
    let my_city2 = City2 {
        name: &city_name[0],
        date_founded: 1995,
    };
    println!("{my_city2:?}");

    let mut billy = Adventurer {
        name: "Biily",
        hit_point: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();
}
