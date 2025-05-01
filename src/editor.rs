use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Default for Editor {
    fn default() -> Self {
        Self {}
    }
}

impl Editor {
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    match event.code {
                        Char(c) => {
                            if c.is_control() {
                                println!("Binary: {0:08b} ASCII: {0:03} \r", c as u8);
                            } else {
                                println!(
                                    "Binary: {0:08b} ASCII: {0:03} Character: {1:#?}\r",
                                    c as u8, c
                                );
                            }
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                Err(err) => println!("Error: {}", err),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}
