use std::io::stdout;
use crossterm::event::{EventStream, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{self, Clear, ClearType};
use futures::{future::FutureExt, StreamExt};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("hello tui");
    println!("press 'q' to exit");

    terminal::enable_raw_mode()?;
    let mut reader = EventStream::new();
    while let Some(e) = reader.next().await {
        let e = e?;
        match e {
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
