fn main() {
    let my_variable = 8;
    let my_reference = &my_variable;
    println!("my_reference {}", my_reference);

    let single_reference = &my_variable;
    let doulbe_reference = &&my_variable;
    let five_reference = &&&&&my_variable;
    println!("single_reference {}", single_reference);
    println!("doulbe_reference {}", doulbe_reference);
    println!("five_reference {}", five_reference);


    let name = "자우림";
    let other_name = String::from("Adrian Fahrenheit Țepeș");

    println!("name {}.nother_name {}", name, other_name);

    let size_of_string = std::mem::size_of::<String>();
    let size_of_i8 = std::mem::size_of::<i8>();
    let size_of_f64 = std::mem::size_of::<f64>();
    let size_of_jaurim = std::mem::size_of_val("자우림");

    let size_of_adrian = std::mem::size_of_val("Adrian Fahrenheit Țepeș");

    println!("A String is Sized and always {size_of_string} bytes.");
    println!("An i8 is Sized and always {size_of_i8} bytes.");
    println!("An f64 is always Sized and {size_of_f64} bytes.");
    println!("But a &str is not Sized: '자우림' is {size_of_jaurim} bytes.");
    println!("And 'Adrian Fahrenheit Țepeș' is {size_of_adrian} bytes - not Sized.");

    /* Retorna ERRO
    let mut country = String::from("UK");
    let ref_one = &country;
    println!("{}", ref_one);
    country = String::from("Brazil");
    println!("{}", ref_one);
    let ref_two = &country;
    println!("{}", ref_two);
    */

    /* Tambem dá ERRO
    let mut country = "UK";
    let ref_one = &country;
    println!("{}", ref_one);
    country = "Brazil";
    println!("{}", ref_one);
    let ref_two = &country;
    println!("{}", ref_two);
    */

    let mut my_mut_number = 8;
    let mut_num_ref = &mut my_mut_number; // Tipo mut i32
    println!("mut_num_ref {}", mut_num_ref);
    println!("*mut_num_ref {}", *mut_num_ref); // Tipo i32
    println!("*mut_num_ref + 10 {}", *mut_num_ref + 10);
    *mut_num_ref += 10;
    println!("*mut_num_ref {}", *mut_num_ref);
    println!("my_mut_number {}", my_mut_number);

}
