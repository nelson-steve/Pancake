use crossterm::event::{read, KeyCode, Event, KeyEventKind, KeyModifiers};

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    crossterm::terminal::enable_raw_mode().ok();

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
