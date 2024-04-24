enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;
    let happiness_level = match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    };
    happiness_level
}

enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

fn main() {
    let my_mood = Mood::Happy;
    let happinesse_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {happinesse_level}");
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("Season as u32 {}", season as u32);
        // println!("Season {}", season as i32); da Erro pois Season nao tem metodos Copy ou Clone !!!!!
    }

    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant, DeadStar];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."),
            size if size >= 80 && size <= 200 => println!("This is a good-sized star."),
            other_size => println!("That star is pretty big! It's {other_size}"),
            // _ => println!("That star is pretty big!"), Poderia ser isso, mas como vamos usar o valor temos q fazer do jeito acima
        }
    }
}
