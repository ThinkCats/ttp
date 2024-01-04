use ratatui::{Frame, layout::Alignment, symbols, widgets::{Block, Borders, BorderType, Paragraph}};
use ratatui::prelude::{Style, Stylize};
use ratatui::widgets::Tabs;

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

    f.render_widget(Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
                        .block(Block::default().borders(Borders::ALL))
                        .style(Style::default().white())
                        .highlight_style(Style::default().yellow())
                        .select(app.tab_state.tab_select as usize)
                        .divider(symbols::DOT)
                    , f.size());

}
