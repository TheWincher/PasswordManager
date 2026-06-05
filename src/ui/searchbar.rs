use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::{App, AppMode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let query = &app.search_query;
    let count = app.filtered_indices.len();

    let cursor = if app.mode == AppMode::Search { "_" } else { "" };

    let text = Line::from(vec![
        Span::styled(" / ", Style::new().fg(Color::Yellow).bold()),
        Span::styled(
            format!("{}{}", query, cursor),
            Style::new().fg(Color::White),
        ),
        Span::raw("   "),
        Span::styled(
            format!("{} résultat(s)", count),
            Style::new().fg(Color::DarkGray),
        ),
    ]);

    let widget = Paragraph::new(text).style(Style::new().bg(Color::Reset));
    frame.render_widget(widget, area);
}
