fn do_something1<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn do_something2<F>(f: F)
where
    F: Fn(), // posso trocar por FnOnce q vai funcionar
{
    f();
}

fn take_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn take_fnmut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn take_fnonce<F: FnOnce()>(f: F) {
    f();
    // f(); nao vai funcionar
}

fn main() {
    let some_vec1 = vec![9, 8, 10];
    do_something1(|| {
        some_vec1
            .into_iter()
            .for_each(|x| println!("The number is: {x}"));
    });

    let some_vec2 = vec![9, 8, 10];
    do_something2(|| {
        some_vec2
            .iter()
            .for_each(|x| println!("The number is: {x}"));
    });

    let mut my_string = String::from("Hello there");

    let print_string = || println!("{my_string}");

    take_fn(print_string);

    let adds_exclamation_and_prints = || {
        my_string.push('!');
        println!("{my_string}")
    };

    take_fnmut(adds_exclamation_and_prints);

    let print_then_drops = || {
        println!("Now dropping {my_string}");
        drop(my_string);
    };
    take_fnonce(print_then_drops);
}
