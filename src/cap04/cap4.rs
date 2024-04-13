fn main() {
    struct FileDirectroy; // unit Struct
    struct ColorRgb(u8, u8, u8); // tuple Struct
    struct SizeAndColor {
        size: u32,
        color: ColorRgb,
    }

    let my_color = ColorRgb(50, 0, 50);
    println!("Tuple Struct my_color_1 {} my_color_2 {} my_color_3 {}", my_color.0, my_color.1, my_color.2);

    let size_and_color = SizeAndColor {
        size: 150,
        color: my_color,
    };

    struct Country {
        population: u32,
        capital: String,
        leader_name: String,
    }
    ;

    let kalmykia = Country {
        population: 500_00,
        capital: String::from("Elista"),
        leader_name: "Batu Khasikov".to_string(),
    };

    enum Climate {
        Tropical,
        Dry,
        Temperate,
        Continental,
        Polar,
    }
    ;

    struct Country2 {
        population: u32,
        capital: String,
        leader_name: String,
        climate: Climate,
    }
    ;
    let kalmykia2 = Country2 {
        population: 500_00,
        capital: String::from("Elista"),
        leader_name: "Batu Khasikov".to_string(),
        climate: Climate::Continental,
    };

    enum ThingsInTheSky {
        Sun,
        Stars,
    };

    fn create_skystate(time: i32) -> ThingsInTheSky {
        match time {
            6..=18 => ThingsInTheSky::Sun,
            _ => ThingsInTheSky::Stars,
        }
    }

    fn check_skystate(state: &ThingsInTheSky) {
        match state {
            ThingsInTheSky::Sun => println!("I can see the sun!"),
            ThingsInTheSky::Stars => println!("I can see the stars!")
        }
    }

    enum ThingsInTheSky2 {
        Sun(String),
        Stars(String),
    };

    fn create_skystate2(time: i32) -> ThingsInTheSky2 {
        match time {
            6..=18 => ThingsInTheSky2::Sun(String::from("I can see the sun")),
            _ => ThingsInTheSky2::Stars(String::from("I can see the stars")),
        }
    }

    fn check_skystate2(state: &ThingsInTheSky2) {
        match state {
            ThingsInTheSky2::Sun(description)=> println!("I can see the sun!"),
            ThingsInTheSky2::Stars(description) => println!("I can see the stars!")
        }
    };

    let time = 28;
    let skystate = create_skystate(time);
    check_skystate(&skystate);
    println!("{time}");

    let skystate2 = create_skystate2(time);
    check_skystate2(&skystate2);
}
