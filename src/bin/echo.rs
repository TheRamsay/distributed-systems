use std::io::{self, Write};

use mlemik::{handle_echo, handle_init, Message};

fn main() {
    let mut buffer = String::new();
    loop {
        if let Ok(_) = io::stdin().read_line(&mut buffer) {
            match serde_json::from_str::<Message>(&buffer) {
                Ok(message) => match message.body.message_type.as_str() {
                    "echo" => serde_json::to_writer(io::stdout(), &handle_echo(&message)).unwrap(),
                    "init" => serde_json::to_writer(io::stdout(), &handle_init(&message)).unwrap(),
                    _ => eprintln!("[ERR] Unknown message type"),
                },
                Err(err) => eprintln!("[ERR] Failed to parse line: {:?}", err),
            }

            io::stdout().write_all(b"\n");
        }

        buffer.clear()
    }
}
