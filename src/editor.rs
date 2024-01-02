use std::time::Duration;

use crossterm::event::{poll, read, KeyCode, Event, KeyEventKind, KeyModifiers};

pub struct Editor {}

impl Editor {
    pub fn run(&self){
        let mut count = 0;
        loop {
            count += 1;
            if let Ok(true) = poll(Duration::from_millis(100)){
                match read().unwrap(){
                    Event::Key(event) => {
                        if event.kind == KeyEventKind::Press {
                            if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('c'){
                                break;
                            }
                            else {
                                println!("{:?}", event);
                            }
                        }
                    },
                    _ => todo!(),
                }
            }
        }
    }

    pub fn default() -> Self {
        Self{}
    }
}

fn die(e: &std::io::Error) {
    panic!("{}", e);
}