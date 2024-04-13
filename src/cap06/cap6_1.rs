use std::collections::{HashMap, HashSet, BinaryHeap, VecDeque};

fn main() {

    println!("\n\nHASHMAP\n\n");

    let book_collection = vec![
        "L'Allemagne Moderne",
        "Le Petit Prince",
        "Eye of the World",
        "Eye of the World",
    ];
    let mut book_hashmap = HashMap::new();
    for book in &book_collection { // Originalmente era book_collection, mas como vamos fazer outro exemplo tive q fazer essa alteracao
        // entry retorna um Enum Entry que tem metodo chamado or_insert
        book_hashmap.entry(book).or_insert(true);
    }
    for (book, true_or_false) in book_hashmap {
        println!("Do we have {book}? {true_or_false}");
    }

    let mut book_hashmap2 = HashMap::new();
    for book in &book_collection {  // Outra forma de usar  book_collection seria criar outro vec igual mas com outro nome
        // entry retorna um Enum Entry que tem metodo chamado or_insert
        let return_value = book_hashmap2.entry(book).or_insert(0); // retorna um referenca mutavel
        *return_value += 1;
    }
    for (book, number) in book_hashmap2 {
        println!("Do we have {book}? {number}");
    }

    println!("\n\nHASHSET\n\n");

    let many_numbers = vec![37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
                            20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
                            46, 33, ];
    println!("How many numbers in the Vec? {}", many_numbers.len());
    let mut number_hashset = HashSet::new();
    for number in many_numbers {
        number_hashset.insert(number);
    }
    let hashset_length = number_hashset.len();
    println!("There are {hashset_length} unique numbers, so we are missing {}.", 50 - hashset_length);
    println!("It does not contain: ");
    for number in 0..=50 {
        if number_hashset.get(&number).is_none() {
            print!("{number} ");
        }
    }

    println!("\n\nBINARYHEAP\n\n");

    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
    let mut heap = BinaryHeap::new();
    for num in many_numbers {
        heap.push(num);
    }
    println!("First item is largest, others are out of order: {heap:?}");
    println!("First item is largest, others are out of order: {heap:?}");
    while let Some(num) = heap.pop() {
        println!("Popped off {num}. Remaining numbers are: {heap:?}");
    }

    println!("\n\nVECDEQUE\n\n");

    let mut my_vec = VecDeque::from(vec![0; 600000]);
    for i in 0..600000 {
        my_vec.pop_front(); // existe o pop_back
    }
}