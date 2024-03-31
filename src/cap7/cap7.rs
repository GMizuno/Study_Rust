struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

struct Animal {
    name: String,
}

trait DogLike {
    fn bark(&self) {
        println!("Woof Woof");
    }

    fn run(&self) {
        println!("The dog is running")
    }
}

impl DogLike for Parrot {
    fn run(&self) {
        println!("{} the parrot is running!!", self.name)
    }
}

trait DogLike2 {
    fn bark(&self);
    fn run(&self);
}

impl DogLike2 for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!!", self.name);
    }

    fn run(&self) {
        println!("{}, stop running!!!", self.name);
    }
}

fn main() {
    let brain = Parrot {
        name: "Brain".to_string(),
    };

    let rover = Dog {
        name: "Rover".to_string(),
    };

    brain.bark();
    brain.run();

    let rex = Animal {
        name: "Rex".to_string(),
    };

    rex.run();
    rex.bark();
}
