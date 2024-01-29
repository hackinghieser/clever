use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.size());
    let details = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main[1]);

    let max_index_length: String = app.rows.len().to_string();
    UnicodeWidthStr::width(max_index_length.as_str());
    let widths = [Constraint::Length(7), Constraint::Max(100)];

    let table = Table::new(app.rows.clone(), widths)
        .style(Style::new().blue())
        .header(Row::new(vec!["Index", "Message"]).style(Style::new().bold()))
        .block(
            Block::default()
                .title("Clever")
                .title_alignment(ratatui::layout::Alignment::Center)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded),
        )
        .style(Style::default().fg(ratatui::style::Color::Yellow))
        .highlight_style(Style::new().reversed());

    f.render_stateful_widget(table, main[0], &mut app.table_state);

    let stats = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title("Types")
        .border_style(Style::default().fg(ratatui::style::Color::Yellow))
        .style(Style::default());

        let chart = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title("Chart")
        .border_style(Style::default().fg(ratatui::style::Color::Yellow))
        .style(Style::default());

    f.render_widget(stats, details[0]);
    f.render_widget(chart, details[1]);
}
