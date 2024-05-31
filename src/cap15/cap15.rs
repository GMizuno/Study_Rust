// EXEMPLO 1
#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
        }
    }
}

// EXEMPLO 2
#[derive(Default, Debug)]
struct Size {
    height: f64,
    length: f64,
    width: f64,
}

fn main() {
    // EXEMPLO 1
    let character_1 = Character::default();
    println!(
        "The character {:?} is {:?} years old",
        character_1.name, character_1.age
    );

    // EXEMPLO 2
    let only_height = Size {
        height: 1.0,
        ..Default::default()
    };
    println!("{only_height:?}");
}
