enum Message {
    // TODO: Implement the message variant types based on their usage below.
    Quit,
    Echo(String),
    Move(Point),
    ChangeColor(u8, u8, u8)
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message variants.
        // Remember: When passing a tuple as a function argument, you'll need extra parentheses:
        // e.g. `foo((t, u, p, l, e))`
        match message {
            Message::Move(point) => {
                println!("Move to specific coordinates (x: 0, y: 0)");
                self.move_position(point)
            }
            Message::Echo(s) => {
                println!("Echoing 'hello world'");
                self.echo(s)
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change color to RGB (0, 0, 0)");
                self.change_color((r, g, b))
            }
            Message::Quit => {
                println!("Quitting application");
                // Process the quit message
                self.quit()
            }
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: String::from("hello world"),
        };

        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert!(state.quit);
        assert_eq!(state.message, "Hello world!");
    }
}
