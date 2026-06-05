use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

use crate::app::{App, FocusedPanel, PasswordStrength};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == FocusedPanel::Detail;
    let border_style = if focused {
        Style::new().fg(Color::Blue)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let block = Block::bordered()
        .title(" Détail ")
        .border_style(border_style);

    // Cas : aucun résultat de recherche
    let Some(entry) = app.selected_entry() else {
        let empty = Paragraph::new("Aucun résultat.")
            .style(Style::new().fg(Color::DarkGray))
            .block(block);
        frame.render_widget(empty, area);
        return;
    };

    let strength_span = match entry.strength {
        PasswordStrength::Strong => Span::styled("● Forte", Style::new().fg(Color::Green)),
        PasswordStrength::Medium => Span::styled("● Moyenne", Style::new().fg(Color::Yellow)),
        PasswordStrength::Weak => Span::styled("● Faible", Style::new().fg(Color::Red)),
    };

    let text = vec![
        Line::from(Span::styled(
            entry.title.clone(),
            Style::new().bold().fg(Color::White),
        )),
        Line::from(Span::styled(
            entry.url.clone(),
            Style::new().fg(Color::Blue),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("LOGIN   ", Style::new().fg(Color::DarkGray)),
            Span::raw(entry.username.clone()),
        ]),
        Line::from(vec![
            Span::styled("PASSWD  ", Style::new().fg(Color::DarkGray)),
            Span::raw("••••••••••  "),
            strength_span,
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("MODIFIÉ ", Style::new().fg(Color::DarkGray)),
            Span::raw(entry.last_modified.clone()),
        ]),
        Line::from(vec![
            Span::styled("TAGS    ", Style::new().fg(Color::DarkGray)),
            Span::raw(entry.tags.join(", ")),
        ]),
    ];

    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
