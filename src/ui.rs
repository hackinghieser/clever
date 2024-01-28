use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    symbols::line,
    widgets::{Block, Borders, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let widths = [Constraint::Length(f.size().width)];
    let mut table_state = TableState::default();
    table_state.select(Some(0));
    let mut rows: Vec<Row> = vec![];
    for line in app.lines.clone() {
        let row = Row::new(vec![line.to_string()]);
        rows.push(row);
    }
    f.render_stateful_widget(
        Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::new().blue())
            .header(
                Row::new(vec!["Message"])
                    .style(Style::new().bold())
                    .bottom_margin(1),
            )
            .block(
                Block::default()
                    .title("Counter App")
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded),
            )
            .style(Style::default().fg(ratatui::style::Color::Yellow))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>"),
        f.size(),
        &mut table_state,
    )
}
