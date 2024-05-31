// EXEMPLO PARA ENTENDER O USO DO PATTERN
#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
    can_use: bool, // USUARIO FINAL PRECISA CONHECER SOBRE ESSE PARAMETRO????
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}

impl Character {
    fn height(mut self, height: u32) -> Self {
        self.height = height;
        self.can_use = false;
        self
    }
    fn weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self.can_use = false;
        self
    }
    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self.can_use = false;
        self
    }
    fn build(mut self) -> Result<Character, String> {
        if self.height < 200 && self.weight < 300 && !self.name.to_lowercase().contains("smurf") {
            self.can_use = true;
            Ok(self)
        } else {
            Err("Could not create character. Characters must have:\n1) Height below 200\n2) Weight below 300\n3) A name that is not Smurf (that is a bad word)"
                .to_string())
        }
    }
}
impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
            can_use: true,
        }
    }
}

fn main() {
    let character_with_smurf = Character::default().name("Lol I am Smurf!!").build();
    let character_too_tall = Character::default().height(400).build();
    let character_too_heavy = Character::default().weight(500).build();
    let okay_character = Character::default()
        .name("Billybrobby")
        .height(180)
        .weight(100)
        .build();
    let character_vec = vec![
        character_with_smurf,
        character_too_tall,
        character_too_heavy,
        okay_character,
    ];

    for charater in character_vec {
        match charater {
            Ok(charater) => println!("{charater:?}\n"),
            Err(charater) => println!("{charater:?}\n"),
        }
    }
}
