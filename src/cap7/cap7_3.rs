#[derive(Clone, Debug)]
struct File(String); // File envolvido por um String

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_bytes = format!("{:?}", self.0.as_bytes());
        println!("Print Display");
        write!(f, "{as_bytes}")
    }
}

fn main() {
    let my_file = File(String::from("I am file content"));
    let my_string = String::from("I am file content");
    println!("{}", my_file.0 == my_string);

    println!("{my_file:?}");
    println!("{my_file}");
}
