fn main() {
    let new_vec = (1..).take(10).collect::<Vec<i32>>();
    println!("{new_vec:?}");

    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .into_iter()
        .skip(3)
        .take(4)
        .collect::<Vec<i32>>();
    println!("{my_vec:?}");

    println!("\n\nITER, ITER_MUT, INTO_ITER\n\n");

    let vector1 = vec![1, 2, 3];
    let mut vector2 = vec![10, 20, 30];
    for num in vector1.iter() {
        // itera sobre uma REFERENCIA
        println!("Printing &i32: {num}");
    }

    println!("{vector1:?}");

    for num in vector1 {
        // itera sobre VALOR, seria o mesmo q vector1.into_iter()
        println!("Printing i32: {num}");
    }

    // println!("{vector1:?}") vector1 sofre um move no segundo for

    for num in vector2.iter_mut() {
        // itera sobre uma REFERENCIA MUTAVEL
        println!("num is {num} before transformation");
        *num *= 10;
        println!("num is now {num}");
    }

    println!("{vector2:?}");

    println!("\n\nMAP\n\n");

    let vector1 = vec![1, 2, 3];
    let mut vector2 = vec![10, 20, 30];

    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>(); // Ref
                                                                         // let vector1_a = vector1.iter().map(|x| x + 1);
    let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>(); // Value
    vector2.iter_mut().for_each(|x| *x += 100); // Ref mutavel

    // println!("{vector1:?}") vector1 sofre um move em vector1.into_iter()
    println!("{:?}", vector1_a);
    println!("{:?}", vector1_b);
    println!("{:?}", vector2);
}
