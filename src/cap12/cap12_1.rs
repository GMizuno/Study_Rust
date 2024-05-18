// EXEMPLO 1
fn takes_fnonce<F: FnOnce()>(f: F) {
    f();
}
fn takes_fnmut<F: FnMut()>(mut f: F) {
    f();
    f();
}

// EXEMPLO 2

fn takes_a_closure_and_does_nothing<F>(f: F)
where
    F: Fn() -> i32,
{
}

fn takes_two_closures_and_does_nothing<F>(first: F, second: F)
where
    F: Fn() -> i32,
{
}

fn takes_two_closures_and_does_nothing_corrigido<F, G>(first: F, second: G)
where
    F: Fn() -> i32,
    G: Fn() -> i32,
{
}

fn main() {
    // EXEMPLO 1
    let mut my_string = String::from("Hello there");
    let prints_string = || {
        println!("{my_string}");
    };
    takes_fnonce(prints_string); // Recebe FnOnce e nao implementa Fn
    takes_fnmut(prints_string); // Recebe FnMut e nao implementa Fn
    let adds_exclamation_and_prints = || {
        my_string.push('!');
        println!("{my_string}");
    };
    takes_fnonce(adds_exclamation_and_prints);
    let prints_then_drops = || {
        println!("Now dropping {my_string}");
        drop(my_string);
    };
    takes_fnonce(prints_then_drops);

    // EXEMPLO 2

    let my_closure = || 9;
    let first_closure = || 9;
    let second_closure = || 9;
    takes_a_closure_and_does_nothing(my_closure);
    // takes_two_closures_and_does_nothing(first_closure, second_closure); // Da erro
    takes_two_closures_and_does_nothing_corrigido(first_closure, second_closure);
}
