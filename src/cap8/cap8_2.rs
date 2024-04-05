fn main() {
    let my_clouser = || println!("This is my closure");
    my_clouser();

    let my_clouser2 = |x: i32| println!("This is my closure with parameters {x}");
    my_clouser2(3);

    let my_long_clouser = || {
        let number = 7;
        let other_number = 32;
        println!(
            "This is my long closure with parameters {}",
            number + other_number
        );
    };
    my_long_clouser();

    let number_one = 6;
    let number_two = 10;
    let my_closure3 = || println!("number_one + number_two = {}", number_one + number_two);
    my_closure3();

    let my_closure4 = |x: i32| {
        println!(
            "number_one + number_two + x = {}",
            number_one + number_two + x
        )
    };
    my_closure4(100);
}
