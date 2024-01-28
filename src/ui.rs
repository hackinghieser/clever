use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let widths = [Constraint::Length(7),Constraint::Max(100)];
    f.render_stateful_widget(
        Table::new(app.rows.clone(), widths)
            .column_spacing(1)
            .style(Style::new().blue())
            .header(Row::new(vec!["Index", "Message"]).style(Style::new().bold()))
            .block(
                Block::default()
                    .title("Counter App")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .style(Style::default().fg(ratatui::style::Color::Yellow))
            .highlight_style(Style::new().reversed()),
        f.size(),
        &mut app.table_state,
    )
}
