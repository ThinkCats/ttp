use ratatui::{
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "
    Press `Esc`, `Ctrl-c` or `q` to stop running . \n
    Press `j` and `k` to increment and decrement the counter. \n
    Counter:{}
    ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Thick),
        )
        .alignment(Alignment::Center),
        f.size(),
    );
}
