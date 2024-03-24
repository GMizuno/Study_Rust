fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("The counter is now: {counter}");
        if counter == 5 {
            break;
        }
    }

    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("The new counter is now: {counter}");
    }

    for number in 0..3 {
        println!("The number is : {number}")
    }

    println!("New loop");

    for number in 0..=3 {
        println!("The number is : {number}")
    }
}