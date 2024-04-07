use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);
    println!("my_mutex = {my_mutex:?}");

    let mut mutex_changer = my_mutex.lock().unwrap(); // mutex vai ficar com lock ate sair do escopo
    println!("my_mutex = {my_mutex:?}"); // aqui esta data: <locked>
    println!("mutex_changer = {mutex_changer:?}");

    *mutex_changer = 6;

    println!("mutex_changer = {mutex_changer:?}");
    println!("my_mutex = {my_mutex:?}");

    println!("\n\nSAIDO DO ESCOPO\n\n");

    let my_mutex = Mutex::new(5);
    println!("my_mutex = {my_mutex:?}");
    {
        let mut mutex_changer = my_mutex.lock().unwrap();
        *mutex_changer = 6;
        println!("my_mutex (dentro do escopo) = {my_mutex:?}");
    }
    println!("mutex_changer = {mutex_changer:?}");
    println!("my_mutex = {my_mutex:?}");

    println!("\n\nUSANDO DROP\n\n");

    let my_mutex = Mutex::new(5);
    println!("{my_mutex:?}");

    let mut mutex_changer = my_mutex.lock().unwrap();
    *mutex_changer = 6;
    drop(mutex_changer);

    println!("{my_mutex:?}");

    println!("\n\nEXEMPLO PROBLEMATICO\n\n");

    let my_mutex = Mutex::new(10);
    println!("{my_mutex:?}");
    let mut mutex_changer = my_mutex.lock().unwrap();
    println!("{mutex_changer:?}");
    let mut other_mutex_changer = my_mutex.lock().unwrap(); // usar try_lock para tentar lockar

    println!("Nuca vai printar {other_mutex_changer:?}");
    println!("Nuca vai printar");

    println!("\n\nSOLUCAO PARA PROBLEMA \n\n");

    // let mut other_mutex_changer = my_mutex.try_lock(); // SOLUCAO

    // match other_mutex_changer {
    //     Ok(value) => println!("The MutexGuard has value {value}"),
    //     _ => println!("Didn't get the lock"),
    // }
}
