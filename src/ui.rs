use ratatui::{
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!(
            "
            Press ESC, C, or q/Q to stop running \n\
            Press j and k to increment and decrement the counter \n\
            Counter: {}
            ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(ratatui::layout::Alignment::Center)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        )
        .style(Style::default().fg(ratatui::style::Color::Yellow))
        .alignment(ratatui::layout::Alignment::Center),
        f.size(),
    )
}
