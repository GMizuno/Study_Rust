fn main() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six");
    } else if my_number == 5 {
        println!("It's five");
    }

    if my_number % 2 == 1 && my_number > 0 {
        println!("It's a positive odd number");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }

    let my_number2: u8 = 5;
    match my_number2 {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"), // Nao roda sem esse caso pois compilador vai procurar por todas as opcoes em u6 (0 ate 255)
    }

    let sky = "cloudy";
    let temperature = "warm";
    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }

    let children = 5;
    let married = true;
    match (children, married) {
        (children, married) if !married => println!("Not married with {children} kids"), // Procuando por married == false
        (children, married) if children == 0 && married => println!("Married but no children"),
        _ => println!("Married? {married}. Number of children: {children}."),
    }

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    match_colors(first);
    match_colors(second);
    match_colors(third);


    match_number(50);
    match_number(13);
    match_number(16);
    match_number(4);
}

fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each color has at least 10"),
    }
}

fn match_number(input: i32) {
    match input {
        number @ 4 => println!("{number} is unlucky in China (sounds ➥close to 死)!"),
        number @ 13 => println!("{number} is lucky in Italy! In ➥bocca al lupo!"),
        number @ 14..=19 => println!("Some other number that ends ➥with -teen: {number}"),
        _ => println!("Some other number, I guess"),
    }
}
