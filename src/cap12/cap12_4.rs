use std::sync::{Arc, Mutex};

fn make_arc(number: i32) -> Arc<Mutex<i32>> {
    Arc::new(Mutex::new(number))
}

fn new_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
    Arc::clone(&input)
}

fn main() {
    // EXEMPLO 1
    let my_number = Arc::new(Mutex::new(0));

    let cloned_1 = Arc::clone(&my_number);
    let cloned_2 = Arc::clone(&my_number);

    let handle1 = std::thread::spawn(move || {
        for _ in 0..5 {
            *cloned_1.lock().unwrap() += 1;
        }
    });
    let handle2 = std::thread::spawn(move || {
        for _ in 0..1000 {
            *cloned_2.lock().unwrap() += 1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("Value is: {my_number:?}");
    println!("Exiting the program!!!!!!");

    // EXEMPLO 2

    let my_number = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![];

    for _ in 0..10 {
        let cloned = Arc::clone(&my_number);
        let handle = std::thread::spawn(move || {
            for _ in 0..5_000 {
                *cloned.lock().unwrap() += 1;
            }
        });
        handle_vec.push(handle);
    }

    handle_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    println!("Value is: {my_number:?}");

    // EXEMPLO 3

    let mut handle_vec = vec![];
    let my_number = make_arc(0);

    for _ in 0..10 {
        let my_number_clone = new_clone(&my_number);
        let handle = std::thread::spawn(move || {
            for _ in 0..45_000 {
                let mut value_inside = my_number_clone.lock().unwrap();
                *value_inside += 1;
            }
        });
        handle_vec.push(handle);
    }

    handle_vec
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
    println!("Value is: {my_number:?}");

    // EXEMPLO 4

    let my_number = Mutex::new(0);
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });
    });
}
