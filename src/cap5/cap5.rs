use std::fmt::Debug;
use std::fmt::Display;
use std::cmp::PartialOrd;
use std::collections::btree_map::Values;

fn return_item<T>(item: T) -> T {
    println!("Here is your item.");
    item
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}")
}

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, input_1: U, input_2: U) {
    println!("{statement}! Is {input_1} greater than {input_2}? {}", input_1 > input_2);
}

fn take_fifth_value(values: Vec<i32>) -> Option<i32> {
    // values[5] da erro se deixar assim
    if values.len() < 5 {
        None
    } else {
        Some(values[4])
    }
}

fn handler_options(my_option: &Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Found a {number}"),
            None => println!("Found a None")
        }
    }
}


fn main() {
    let console = return_item("Nintendo");
    let jogo = return_item("Zelda");

    println!("Vc tem {console} e {jogo}");
    print_item(console);
    compare_and_display("Listen up", 1, 3);

    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    let r1 = take_fifth_value(small.clone());
    let r2 = take_fifth_value(big);
    println!("Got it {:?}, {:?}", r1, r2);

    let mut option_vec = Vec::new();
    option_vec.push(r1);
    option_vec.push(r2);

    handler_options(&option_vec);
    // println!("{take_fifth_value:?}") Erro Option nao tem Debug

    for vec in vec![small, big] {
        let inside_number = take_fifth_value(vec);
        if inside_number.is_some() {
            println!(" we got: {}", inside_number.unwrap())
        } else {
            println!("We got nothing");
        }

    }
}