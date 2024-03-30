fn main() {
    let my_vec = vec![2, 3, 4];
    let get_one = my_vec.get(0); // Verifica se existe esse indice
    let get_two = my_vec.get(10);
    println!("{:?}", get_one);
    println!("{:?}", get_two);

    let my_vec = vec![2, 3, 4];

    for i in 0..10 {
        match my_vec.get(i) {
            Some(number) => println!(" the number is {number}"),
            None => {}
        }
    }

    for i in 0..10 {
        if let Some(number) = my_vec.get(i) {
            println!("The number is {number}")
        }
    }

    for i in 0..10 {
        if let Some(number) = my_vec.get(i) {
            println!("The number is {number}")
        }
        let Some(number) = my_vec.get(i) else {
            continue;
        };
        println!("The number is: {number}")
    }

    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athenas", "sunny", "not humid", "20", "10", "50"],
    ];
    for mut city in weather_vec {
        println!("For the city of {}:", city[0]);
        while let Some(information) = city.pop() { // city.pop remove um item de vec, ou seja, continua ate para de remover
            if let Ok(number) = information.parse::<i32>() {
                println!("The number is: {number}");
            }
        }
    }
}
