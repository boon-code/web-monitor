use std::io::stdout;
use crossterm::event::{read, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{self, Clear, ClearType};
use anyhow::Result;

fn main() -> Result<()> {
    println!("hello tui");
    println!("press 'q' to exit");

    terminal::enable_raw_mode()?;

    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => {
                println!("{:?}", event);
                if event.code == KeyCode::Char('q') {
                    break;
                }
            },
            _ => {},
        }
    }

    terminal::disable_raw_mode()?;
    println!("");
    execute!(stdout(), Clear(ClearType::All))?;

    Ok(())
}
