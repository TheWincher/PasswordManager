use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState},
};

use crate::app::{
    App, FocusedPanel,
    audit::{self, PasswordStrength},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == FocusedPanel::Entries;
    let border_style = if focused {
        Style::new().fg(Color::Blue)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let entries = app.visible_entries();

    // Titre dynamique selon recherche
    let title = if app.search_query.is_empty() {
        " Entrées ".to_string()
    } else {
        format!(" Entrées — \"{}\" ", app.search_query)
    };

    let items: Vec<ListItem> = entries
        .iter()
        .map(|e| {
            let dot = match audit::password_strength(&e.password) {
                PasswordStrength::Strong => Span::styled("● ", Style::new().fg(Color::Green)),
                PasswordStrength::Medium => Span::styled("● ", Style::new().fg(Color::Yellow)),
                PasswordStrength::Weak => Span::styled("● ", Style::new().fg(Color::Red)),
            };
            ListItem::new(Line::from(vec![dot, Span::raw(e.title.clone())]))
        })
        .collect();

    let mut state = ListState::default();
    if !entries.is_empty() {
        state.select(Some(app.selected_entry));
    }

    let list = List::new(items)
        .block(Block::bordered().title(title).border_style(border_style))
        .highlight_style(Style::new().bg(Color::Blue).fg(Color::White).bold())
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut state);
}
