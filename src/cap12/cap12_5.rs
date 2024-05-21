use std::sync::mpsc::channel;

fn main() {
    // EXEMPLO 1
    let (sender, receiver) = channel();
    sender.send(5).unwrap();
    let n = receiver.recv().unwrap();
    println!("Number {n}");

    // EXEMPLO 2
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();
    std::thread::spawn(move || {
        sender.send("Send a &str this time").unwrap();
        sender.send("Send a &str this time").unwrap();
    });
    std::thread::spawn(move || {
        sender_clone.send("And here is another &str").unwrap();
        sender_clone.send("And here is another &str").unwrap();
    });
    while let Ok(res) = receiver.recv() {
        println!("{res}");
    }
}
