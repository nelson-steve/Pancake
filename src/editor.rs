use std::time::Duration;

use crossterm::event::{poll, read, KeyCode, Event::*, KeyEventKind, KeyModifiers};

pub struct Editor {}

impl Editor {
    pub fn run(&self){
        loop {
            let mut c = None;
            if let Ok(true) = poll(Duration::from_millis(100)) {
                if let Ok(event) = read() {
                    if let Key(key_event) = event {
                        c = Some(key_event);
                    }
                }
            }
            if let Some(c) = c {
                if c.kind == KeyEventKind::Press {
                    if c.modifiers.contains(KeyModifiers::CONTROL) && c.code == KeyCode::Char('c') {
                        break;
                    } else {
                        println!("{c:?}");
                    }
                }
            } else {
                println!("no key\r");
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