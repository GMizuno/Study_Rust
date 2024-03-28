#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    fn new_cat() -> Self { // Self == Animla (Type)
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn check_type(&self) { // Acessando objeto (self) atraves de referencia
        match self.animal_type {
            AnimalType::Dog => println!("The Animal is a dog!!!!"),
            AnimalType::Cat => println!("The Animal is a cat!!!!"),
        }
    }
    fn change_to_cat(&mut self) { // Acessando objeto de forma mutavel
        self.animal_type = AnimalType::Cat;
        println!("Changed animal to cat! Now it's {self:?}")
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed animal to dog! Now it's {self:?}");
    }

    fn change_age(&mut self, age: u8) {
        self.age = age;
        println!("Changed animal age! Now it's {}", self.age.to_string());
    }

    fn check_age_and_type(&self) {
        // let t = self.animal_type; Da erro de Borrow, t "toma" o valor de self.animal_type
        println!("The Animal is a {:?} with age  {}!!!!", self.animal_type, self.age);
    }
}

fn main() {
    let mut new_animal = Animal::new_cat();
    println!("Animal Type {new_animal:?}");
    new_animal.check_type();
    new_animal.check_age_and_type();
    new_animal.change_to_cat();
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.change_age(100);
    new_animal.change_age(50);
    new_animal.check_age_and_type()
}

