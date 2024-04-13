use std::collections::HashMap;

fn main() {
    (1..=3).for_each(|num| println!("{num}"));

    (1..=3).for_each(|num| {
        println!("Got a {num}!");
        if num % 2 == 0 {
            println!("It's even")
        } else {
            println!("It's odd")
        };
    });

    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if let Some(val) = my_vec.get(2) {
            val
        } else {
            &0
        }
    });
    println!("{fourth}");

    let char_vec = vec!['z', 'y', 'x'];

    char_vec
        .iter()
        .enumerate()
        .for_each(|(index, c)| println!("Index {index} is: {c}"));

    let num_vec = vec![2, 4, 6];

    num_vec
        .iter()
        .enumerate()
        .for_each(|(index, c)| println!("Index {index} is: {c}"));

    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];

    let number_word_hashmap = some_keys
        .into_iter()
        .zip(some_values.into_iter())
        .collect::<HashMap<_, _>>();

    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );

    println!("number_word_hashmap: {:?}", number_word_hashmap);
}
