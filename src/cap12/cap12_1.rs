fn takes_fnonce<F: FnOnce()>(f: F) {
    f();
}
fn takes_fnmut<F: FnMut()>(mut f: F) {
f();
f(); }
fn main() {
    let mut my_string = String::from("Hello there");
    let prints_string = || {
println!("{my_string}");
takes_fnonce takes an FnOnce, and Fn implements FnOnce. No problem.
takes_fnmut takes an FnMut, and Fn implements FnMut. Once again, no problem.
};
takes_fnonce(prints_string);
takes_fnmut(prints_string);
let adds_exclamation_and_prints = || {
    my_string.push('!');
    println!("{my_string}");
};
    takes_fnonce(adds_exclamation_and_prints);
    let prints_then_drops = || {
        println!("Now dropping {my_string}");
        drop(my_string);
    takes_fnonce(prints_then_drops);
}
