use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    widgets::{block, Block, Borders, Paragraph, Row, Table, Wrap},
    Frame,
};

struct Detail {
    timestap: String,
    message: String,
    level: String,
    exception: String,
    event_id: String,
    render_count: u16,
}

use crate::{app::App, clef::ClefLine};

pub fn render(app: &mut App, f: &mut Frame) {
    let widths = [
        Constraint::Length(20),
        Constraint::Max(5),
        Constraint::Percentage(100),
    ];
    let mut clef_rows: Vec<Row> = vec![];
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.size());

    let detail_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(79)])
        .split(main[1]);

    let detail_header = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(detail_area[0]);

    let detail_footer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .split(detail_area[1]);

    for line in &app.lines {
        clef_rows.push(line.row.clone());
    }

    let selected_row: &ClefLine = app.lines.get(app.table_state.selected().unwrap()).unwrap();
    let detail: Detail = Detail {
        timestap: selected_row.time.to_string(),
        message: selected_row.template.to_string(),
        level: selected_row.level.to_string(),
        exception: selected_row.exception.to_string(),
        event_id: selected_row.eventid.to_string(),
        render_count: 3u16,
    };
    let render_count = format!("Count: {}", detail.render_count.to_string());
    let table = Table::new(clef_rows, widths)
        .style(Style::new().blue())
        .column_spacing(0)
        .header(Row::new(vec!["Time", "Log", "Message Template"]).style(Style::new().bold()))
        .block(
            Block::default()
                .title("Clever")
                .title(
                    block::Title::from(app.file_path.as_str())
                        .position(block::Position::Top)
                        .alignment(ratatui::layout::Alignment::Left),
                )
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

    f.render_widget(stats, main[1]);

    let status_details = Paragraph::new(format!(
        "{}|{}    {}   {}  {}",
        detail.timestap, detail.level, detail.exception, detail.event_id, render_count
    ))
    .style(Style::default().fg(ratatui::style::Color::Yellow))
    .block(Block::new().padding(block::Padding { left: 1, right: 1, top: 1, bottom: 0 }));

    f.render_widget(status_details, detail_header[0]);

    let rendered_message = Paragraph::new(detail.message)
        .style(Style::default().fg(ratatui::style::Color::Yellow))
        .wrap(Wrap { trim: false })
        .block(Block::new().padding(block::Padding {
            left: 1,
            right: 1,
            top: 0,
            bottom: 1,
        }));
    f.render_widget(rendered_message, detail_footer[0]);
}
