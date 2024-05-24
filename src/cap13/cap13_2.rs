// Exemplo 1
fn just_takes_a_variable<T>(item: T) {}

// Exemplo 2
#[derive(Debug)]
struct Holder {
    next_holder: Option<Box<Holder>>,
}

// Exemplo 3
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorOne;
impl Error for ErrorOne {}

impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the first error!")
    }
}

#[derive(Debug)]
struct ErrorTwo;
impl Error for ErrorTwo {}

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
    match input {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine to me".to_string()),
    }
}

fn main() {
    // Exemplo 1
    let my_number = 1;
    just_takes_a_variable(my_number);
    just_takes_a_variable(my_number);

    let my_box = Box::new(1);
    just_takes_a_variable(my_box.clone()); // Erro nao tem Clone
    just_takes_a_variable(my_box);

    let my_box = Box::new(1);
    let an_int = *my_box;
    println!("{my_box}, {an_int}");

    // Exemplo 2
    let x = Holder {
        next_holder: Some(Box::new(Holder {
            next_holder: Some(Box::new(Holder { next_holder: None })),
        })),
    };
    println!("{x:#?}");

    // Exemplo 3
    let vec_of_u8s = vec![0_u8, 1, 80, 0, 11, 1];
    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(massage) => println!("{}", massage),
        }
    }
}
