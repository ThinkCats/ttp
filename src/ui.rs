use ratatui::{Frame, layout::Alignment, symbols, widgets::{Block, Borders, BorderType, Paragraph}};
use ratatui::layout::Direction;
use ratatui::prelude::{Constraint, Layout, Style, Stylize};
use ratatui::widgets::Tabs;

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let main_layout = Layout::new(Direction::Vertical,
                                  [Constraint::Length(1),
                                      Constraint::Min(0),
                                      Constraint::Length(1)
                                  ])
        .split(f.size());

    let title_bar = header_tab(app);
    f.render_widget(title_bar, main_layout[0]);

    let status_bar = Block::new().borders(Borders::TOP).title("status bar");
    f.render_widget(status_bar, main_layout[2]);

    let inner_layout = Layout::new(Direction::Horizontal,
                                   [Constraint::Percentage(50), Constraint::Percentage(50)],
    ).split(main_layout[1]);
    let left_content = header_counter(app);
    f.render_widget(left_content, inner_layout[0]);
    let right_content = Block::new().borders(Borders::ALL).title("right");
    f.render_widget(right_content, inner_layout[1]);
}

fn header_counter<'a>(app: &App) -> Paragraph<'a> {
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
                // .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Thick),
        )
        .alignment(Alignment::Center)
}

fn header_tab(app: &App) -> Tabs {
    Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
        // .block(Block::new().borders(Borders::ALL))
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(app.tab_state.tab_select as usize)
        .divider(symbols::DOT)
}