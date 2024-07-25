#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Quit,
    Echo(String),
    Move{x: u8, y: u8},
    ChangeColor(u32, u32, u32)
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
