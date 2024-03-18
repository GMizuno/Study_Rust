use crate::cap1::cap1::add;

mod cap1;

fn main() {
    println!("Hello, world!");
    let x =1;
    let y=2;
    println!("Adding the following numbers {} and {}, then the reuslt is ? {}", x, y, add(y, x));
}
