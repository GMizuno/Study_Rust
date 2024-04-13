fn main() {
    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{country} {country_ref}");

    let mut country2 = String::from("Austria");

    add_country(&country2);
    println!("country2 {country2}");

    let mut country2_2 = String::from("Austria");
    add_country2(&mut country2_2);
    println!("country2_2 {country2_2}");

    add_country3(country2);
    // println!("country2 {country2}"); DA ERROR !!!!!!!!!!!

    let my_number = 8; // Interger implements Copy
    print_number(my_number);
    print_number(my_number);

    let country3 = String::from("Austria");

    add_country4(country3.clone());
    println!("country3 {country3}");
    // add_country(country);

    println!("행 in hexdecimal is {:X}", '행' as u32);
    println!("H in hexdecimal is {:X}", 'H' as u32);
    println!("居 in hexdecimal is {:X}", '居' as u32);
    println!("い in hexdecimal is {:X}", 'い' as u32);
    println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    println!("{: ^15}{: ^15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}

fn add_country(country_name: &String) {
    println!("Now it says: {country_name}");
}

fn add_country2(country_name: &mut String) {
    country_name.push_str(" Hungary");
    println!("country2: {country_name}");
}

fn add_country3(mut country_name_new: String) {
    country_name_new.push_str(" Hungary");
    println!("country3: {country_name_new}");
}

fn print_number(number: i32) {
    println!("{}", number);
}

fn add_country4(country_name_clone: String) {
    println!("Now it says: {country_name_clone}");
}
