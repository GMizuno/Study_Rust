use std::sync::RwLock;

fn main() {
    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    println!("{read1:?}, {read2:?}");

    // let write1 = my_rwlock.write().unwrap(); // deadlock

    drop(read1);
    drop(read2);

    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 6;
    println!("{:?}", my_rwlock);

    println!("\n\nEXEMPLO COM TRY_LOCK\n\n");

    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    println!("{read1:?}, {read2:?}");

    let mut write1 = my_rwlock.try_write();

    match write1 {
        Ok(mut value) => {
            *value += 10;
            println!("Value {value}")
        }
        Err(_) => println!("Couldn't get write access, sorry"),
    };

    println!("\n\nEXEMPLO 2 COM TRY_LOCK\n\n");

    let my_rwlock2 = RwLock::new(5);
    let mut write2 = my_rwlock2.try_write();

    println!("{write2:?}");

    match write2 {
        Ok(mut value) => {
            println!("Value was {value}");
            *value += 10;
            println!("Now Value is {value}")
        }
        Err(_) => println!("Couldn't get write access, sorry"),
    };
}
