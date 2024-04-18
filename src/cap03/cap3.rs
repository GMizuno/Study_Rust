fn main() {
    let array1 = ["One", "Two"];
    let array2 = ["One", "Two", "Five"];

    let my_array = ["a"; 5];
    println!("My Array {:?}", my_array);

    let my_buffer_array = [0u8; 640];
    println!("My Array {:?}", my_buffer_array);

    let my_number = [0, 10, -2000];
    println!("{}", my_number[1]);

    let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_to_five = &array_of_ten[2..5];
    let start_at_one = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];
    let two_to_five_inclusive = &array_of_ten[2..=5];
    println!("Two to five: {two_to_five:?}");
    println!("Start at one: {start_at_one:?}");
    println!("End at five: {end_at_five:?}");
    println!("Everything: {everything:?}");
    println!("Two to five (inclusive): {two_to_five_inclusive:?}");

    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);
    println!("My Vector {my_vec:?}");

    let my_vec1 = vec![8, 10, 10];
    println!("My Vector2 {my_vec1:?}");
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let vec_three_to_five = &vec_of_ten[2..5];
    let vec_start_at_two = &vec_of_ten[1..];
    let vec_end_at_five = &vec_of_ten[..5];
    let vec_everything = &vec_of_ten[..];

    println!("Three to five: {vec_three_to_five:?}");
    println!("start at two: {vec_start_at_two:?}");
    println!("end_at_five: {vec_end_at_five:?}");
    println!("Everything: {vec_everything:?}");

    let mut my_vec_c: Vec<String> = Vec::new();
    println!("my_vec_c {}", my_vec_c.capacity());
    println!("my_vec {}", my_vec.capacity());
    println!("my_vec1 {}", my_vec1.capacity());

    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!("Inside the tuple is: First item: {:?}", random_tuple.0);
    println!("Second item: {:?}", random_tuple.1);
    println!("Third item: {:?}", random_tuple.2);
    println!("Fourth item: {:?}", random_tuple.3);
    println!("Fifth item: {:?}", random_tuple.4);
    println!("Sixth item: {:?}", random_tuple.5);

    let strings = ("one".to_string(), "two".to_string(), "three".to_string());
    let (a, b, c) = strings; // Destructuring
    println!("{b}");
    // println!("{strings:?}"); Da erro String nao tem Copy

    let integers = (1, 2, 3);
    let (a, b, c) = integers; // Destructuring
    println!("{b}");
    println!("{integers:?}"); // Tuple tem Copy
}
