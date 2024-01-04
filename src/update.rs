use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App, Mode};

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.mode {
        Mode::Normal => {
            normal_event(app, key_event)
        }
        Mode::Insert => {}
        Mode::Processing => {}
        Mode::Help => {}
    }
}

fn normal_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Up | KeyCode::Char('j') => app.increment_counter(),
        KeyCode::Down | KeyCode::Char('k') => app.decrement_counter(),
        KeyCode::Tab => app.switch_tab(true),
        KeyCode::Backspace => app.switch_tab(false),
        _ => {}
    }
}