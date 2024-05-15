fn main() {
    let mut join_handles = vec![];
    for num in 0..10 {
        let handle = std::thread::spawn(|| {
            println!("I am printing something");
        });
        join_handles.push(handle);
    }
    for handle in join_handles {
        handle.join().unwrap();
    }

    // let mut busy_work = vec![];
    // // loop para segurar thread main
    // for _ in 0..1_000_000 {
    //     busy_work.push(9);
    //     busy_work.pop();
    // }

    let mut join_handles = vec![];
    for num in 0..10 {
        let handle = std::thread::spawn(move || {
            println!("Inside thread number: {num}");
        });
        join_handles.push(handle);
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
}
