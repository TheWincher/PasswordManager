use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::{
    app::{App, AppMode},
    ui::utils::key_hint,
};

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let hints = match app.mode {
        AppMode::Locked => vec![key_hint("↵", "déverrouiller"), key_hint("q", "quitter")],
        AppMode::Normal => vec![
            key_hint("Tab", "panel"),
            key_hint("j/k", "naviguer"),
            key_hint("n", "nouveau"),
            key_hint("e", "éditer"),
            key_hint("y", "copier login"),
            key_hint("/", "rechercher"),
            key_hint("a", "audit"),
            key_hint("l", "verrouiller"),
            key_hint("q", "quitter"),
        ],
        AppMode::Popup => vec![
            key_hint("Tab", "champ suivant"),
            key_hint("S-Tab", "précédent"),
            key_hint("↵", "enregistrer"),
            key_hint("Esc", "annuler"),
        ],
        AppMode::Search => vec![
            key_hint("↵", "valider"),
            key_hint("Esc", "annuler"),
            key_hint("⌫", "effacer"),
        ],
        AppMode::Audit => vec![
            key_hint("Tab", "panel"),
            key_hint("j/k", "naviguer"),
            key_hint("e", "éditer entrée"),
            key_hint("Esc", "retour"),
        ],
    };

    let mut spans = vec![];
    for (i, h) in hints.into_iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(h);
    }

    let text = Paragraph::new(Line::from(spans)).style(Style::new().bg(Color::DarkGray));
    frame.render_widget(text, area);
}
