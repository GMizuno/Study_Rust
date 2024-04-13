use std::num::ParseIntError;

fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parser_number = input
        .parse::<i16>()?
        .to_string()
        .parse::<u32>()?
        .to_string()
        .parse::<i32>()?;
    println!("Number parsed sucessfully into {parser_number}");
    Ok(parser_number)
}

fn main() {
    let fail = " adasda".parse::<i32>();
    // fail.bdasdba(); //fail.bdasdba() method not found in `Result<i32, ParseIntError>`
    let p = parse_and_log_str("as");
    println!("Result: {p:?}");

    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in &str_vec {
        let parsed = parse_and_log_str(item);
        println!("Result: {parsed:?}");
    }

    println!("\n\nMULTIPLE PARSED\n\n");
    for item in &str_vec {
        let parsed = parse_str(item);
        println!("Parsed {parsed:?}");
    }
}