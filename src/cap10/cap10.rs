fn prints_str(my_str: &str) {
    println!("{my_str}");
}

fn restunrs_str() -> &'static str {
    let my_String = String::from("I am a string");
    "I am a str"
}

fn main() {
    let my_String = String::from("I am a string");
    //prints_str(my_String); // Da erro pois eh esperado um referencia
    let my_str = restunrs_str();
    println!("{my_str}")
}
