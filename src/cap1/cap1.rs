fn cap1() {
    let first_letter = 'A';
    let space = ' ';
    let other_language_char = 'Ꮔ';
    let cat_face = ''; // era para ser um emoji

    println!("{}", first_letter);
    println!("{}", space);
    println!("{}", other_language_char);
    println!("{}", cat_face);

    // let my_number = 100; vai dar error pois my_number vais er i32
    let my_number = 100;
    println!("{}", my_number as u8 as char);

    let my_number2 = 600;
    println!("{}", my_number2 as u8 as char);

    let my_number2: u8 = 100;
    println!("{}", my_number2 as char);

    println!("Size of a char: {} bytes", std::mem::size_of::<char>());
    println!("Size of a: {} bytes", "a".len());
    println!("Size of ß: {} bytes", "ß".len());
    println!("Size of 国: {} bytes", "国".len());

    let str1 = "Hello!";
    let str2 = "안녕!";
    println!("str1 is ({}) {} bytes.", str1, str1.len());
    println!("str2 is ({}) {} bytes.", str2, str2.len());

    println!("{:?}", "a".as_bytes());
    println!("{:?}", "ß".as_bytes());
    println!("{:?}", "国".as_bytes());

    let str1 = "Hello!";
    println!(
        "str1 ({}) is {} bytes, with this represetation {:?} and also {} characters.",
        str1,
        str1.len(),
        str1.chars(),
        str1.chars().count()
    );
    let str2 = "안녕!";
    println!(
        "str2 ({}) is {} bytes, with this represetation {:?} but only {} characters.",
        str2,
        str2.len(),
        str2.chars(),
        str2.chars().count()
    );

    let small_number = 10u8; // poderia ser 10_u8 e 100_000_000_i32
    println!("{}", small_number);

    let my_float = 5.0;

    {
        let my_other_float = 5.0;
    }
    /* 
        let third_float = my_float + my_other_float;
        Vai retorna erro relacionado a lifetime
        help: the binding `my_other_float` is available in a different scope in the same function
    */

    println!("Hello, world number {}!", give_number());

    multiply(8, 9);
    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);
}

fn give_number() -> i32 {
    8
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}
