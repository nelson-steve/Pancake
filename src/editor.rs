use crossterm::event::{read, KeyCode, Event, KeyEventKind, KeyModifiers};

pub struct Editor {}

impl Editor {
    pub fn run(&self){
        loop {
            match read().unwrap(){
                Event::Key(event) => {
                    if event.kind == KeyEventKind::Press {
                        if event.modifiers.contains(KeyModifiers::CONTROL) && event.code == KeyCode::Char('q'){
                            break;
                        }
                        if event.modifiers.contains(KeyModifiers::CONTROL){
                            println!("ctr pressed with key: {:?}", event.code);
                        }
                        else {
                            println!("{:?}", event.code);
                        }
                    }
                },
                _ => todo!(),
            }
        }
    }

    pub fn default() -> Self {
        Editor{}
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}