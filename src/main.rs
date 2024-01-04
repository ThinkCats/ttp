use std::io::stderr;

use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::{app::App, event::EventHandler, tui::Tui, update::update};

pub mod app;
pub mod event;
pub mod tui;
pub mod ui;
pub mod update;

fn main() -> Result<()> {
    println!("start ...");

    let mut app = App::new();
    app.init_tab(4);
    let backend = CrosstermBackend::new(stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            event::Event::Tick => {}
            event::Event::Key(key_event) => update(&mut app, key_event),
            event::Event::Mouse(_) => {}
            event::Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;

    Ok(())
}
