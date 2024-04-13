fn cap11() {
    // Vai retonar um erro !!!!!
    // {
    //     let my_number = 8;
    // }
    // println!("Hello, number {}", my_number);

    let my_number = {
        let second_number = 8;
        second_number + 9
    };

    println!("My number is: {}", my_number);

    println!("The smallest i8: {} The biggest i8: {}", i8::MIN, i8::MAX);
    println!("The smallest u8: {} The biggest u8: {}", u8::MIN, u8::MAX);

    println!(
        "The smallest i16: {} The biggest i16: {}",
        i16::MIN,
        i16::MAX
    );
    println!(
        "The smallest u16: {} and the biggest u16: {}",
        u16::MIN,
        u16::MAX
    );
    println!(
        "The smallest i32: {} The biggest i32: {}",
        i32::MIN,
        i32::MAX
    );
    println!(
        "The smallest u32: {} The biggest u32: {}",
        u32::MIN,
        u32::MAX
    );
    println!(
        "The smallest i64: {} The biggest i64: {}",
        i64::MIN,
        i64::MAX
    );
    println!(
        "The smallest u64: {} The biggest u64: {}",
        u64::MIN,
        u64::MAX
    );
    println!(
        "The smallest i128: {} The biggest i128: {}",
        i128::MIN,
        i128::MAX
    );
    println!(
        "The smallest u128: {} The biggest u128: {}",
        u128::MIN,
        u128::MAX
    );

    let my_number_immutable = 8;
    println!("{}", my_number_immutable);
    let mut my_number_mutable = 8;
    my_number_mutable = 10;
    println!("{}", my_number_mutable);

    let my_number_shadowing = 8;
    println!("{}", my_number_shadowing);
    let my_number_shadowing = 8.9;
    println!("{}", my_number_shadowing);

    let my_number2 = 10;
    println!("{}", my_number2);
    {
        let my_number2 = 1111.2;
        println!("{}", my_number2);
    }
    println!("{}", my_number2);
}
