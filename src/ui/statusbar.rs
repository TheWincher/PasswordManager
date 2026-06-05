use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::app::{App, AppMode};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let (mode_text, mode_style) = match app.mode {
        AppMode::Locked => (
            " LOCKED ",
            Style::new().bg(Color::Red).fg(Color::White).bold(),
        ),
        AppMode::Normal => (
            " NORMAL ",
            Style::new().bg(Color::Blue).fg(Color::Black).bold(),
        ),
        AppMode::Popup => (
            " INSERT ",
            Style::new().bg(Color::Green).fg(Color::Black).bold(),
        ),
        AppMode::Search => (
            " SEARCH ",
            Style::new().bg(Color::Yellow).fg(Color::Black).bold(),
        ),
        AppMode::Audit => (
            " AUDIT  ",
            Style::new().bg(Color::Magenta).fg(Color::Black).bold(),
        ),
    };

    let text = Paragraph::new(Line::from(vec![
        Span::styled(mode_text, mode_style),
        Span::raw("  vault: "),
        Span::styled("personal.vault", Style::new().bold()),
        Span::raw("                    "),
        Span::styled("42 entries  🔒", Style::new().fg(Color::DarkGray)),
    ]))
    .style(Style::new().bg(Color::DarkGray));

    frame.render_widget(text, area);
}
