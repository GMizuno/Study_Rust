use std::fmt::Display;
// EXEMPLO 1
fn prints_it(input: impl Into<String> + Display) {
    println!("You can print many things, include {input}");
}

// EXEMPLO 2

fn prints_it_impl_trait(input: impl Display) {
    println!("You can print many things, include {input}");
}

fn prints_it_regular_generic<T: Display>(input: T) {
    println!("You can print many things, include {input}");
}

// EXEMPLO 3 --> Da erro
// fn gives_higher(one: impl PartialOrd + Display, two: impl PartialOrd + Display) {
//     let higher = if one > two { one } else { two };
//     println!("{higher} is higher");
// }

fn main() {
    // EXEMPLO 1
    let name = "Toun";
    let string_name = String::from("Toun");
    prints_it(name);
    prints_it(string_name);

    // EXEMPLO 2
    prints_it_regular_generic::<u8>(100);
    prints_it_impl_trait(100);
    prints_it_impl_trait(100u8);

    // EXEMPLO 3
    // gives_higher(8, 10); Da erro
}
