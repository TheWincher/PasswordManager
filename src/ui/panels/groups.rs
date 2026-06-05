use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState},
};

use crate::app::{App, FocusedPanel};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == FocusedPanel::Groups;
    let border_style = if focused {
        Style::new().fg(Color::Blue)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app
        .groups
        .iter()
        .map(|g| ListItem::new(format!("{} {}", g.icon, g.name)))
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_group));

    let list = List::new(items)
        .block(
            Block::bordered()
                .title(" Groupes ")
                .border_style(border_style),
        )
        .highlight_style(Style::new().bg(Color::Blue).fg(Color::White).bold())
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut state);
}
