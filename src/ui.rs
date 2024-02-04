use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    widgets::{block, Block, Borders, Row, Table},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let widths = [Constraint::Length(20), Constraint::Max(5), Constraint::Percentage(100)];
    let mut clef_rows: Vec<Row> = vec![];

    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.size());
    let details = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60)])
        .split(main[1]);

    
    
    for line in &app.lines {
        clef_rows.push(line.row.clone());
    }
    
    let table = Table::new(clef_rows, widths)
        .style(Style::new().blue())
        .column_spacing(0)
        .header(Row::new(vec!["Time", "Log", "Message Template"]).style(Style::new().bold()))
        .block(
            Block::default()
                .title("Clever")
                .title(block::Title::from(app.file_path.as_str()).position(block::Position::Top).alignment(ratatui::layout::Alignment::Left))
                .title_position(ratatui::widgets::block::Position::Top)
                .title_alignment(ratatui::layout::Alignment::Center)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .title_style(Style::default().fg(ratatui::style::Color::Yellow)),
        )
        .style(Style::default().fg(ratatui::style::Color::Yellow))
        .highlight_style(Style::default().reversed());

    f.render_stateful_widget(table, main[0], &mut app.table_state);

    let stats = Block::default()
        .borders(Borders::ALL)
        .title(block::Title::from("Detail").position(block::Position::Top))
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title("Quit:'Q'  Details: 'D'  Sort: 'S")
        .title_position(ratatui::widgets::block::Position::Bottom)
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .title_alignment(ratatui::layout::Alignment::Left)
        .title_style(Style::default().fg(ratatui::style::Color::Yellow))
        .border_style(Style::default().fg(ratatui::style::Color::Yellow))
        .style(Style::default());
    f.render_widget(stats, details[0]);
}
