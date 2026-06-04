use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::app::{App, FocusedPanel, PasswordStrength};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // ── Layout global : statusbar haut + contenu + keybinds bas ──
    let root = Layout::vertical([
        Constraint::Length(1),  // statusbar
        Constraint::Min(0),     // contenu principal
        Constraint::Length(1),  // barre de raccourcis
    ])
        .split(area);

    render_statusbar(frame, root[0]);
    render_main(frame, app, root[1]);
    render_keybinds(frame, root[2]);
}

// ── Statusbar ─────────────────────────────────────────────────
fn render_statusbar(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new(
        Line::from(vec![
            Span::styled(" NORMAL ", Style::new().bg(Color::Blue).fg(Color::Black).bold()),
            Span::raw("  vault: "),
            Span::styled("personal.vault", Style::new().bold()),
            Span::raw("                    "),
            Span::styled("42 entries  🔒", Style::new().fg(Color::DarkGray)),
        ])
    )
        .style(Style::new().bg(Color::DarkGray));
    frame.render_widget(text, area);
}

// ── Layout 3 colonnes ────────────────────────────────────────
fn render_main(frame: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::horizontal([
        Constraint::Length(20),   // groupes
        Constraint::Min(25),      // entrées
        Constraint::Length(38),   // détail
    ])
        .split(area);

    render_groups(frame, app, cols[0]);
    render_entries(frame, app, cols[1]);
    render_detail(frame, app, cols[2]);
}

// ── Panel Groupes ─────────────────────────────────────────────
fn render_groups(frame: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == FocusedPanel::Groups;
    let border_style = if focused {
        Style::new().fg(Color::Blue)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app.groups.iter().map(|g| {
        ListItem::new(format!("{} {}", g.icon, g.name))
    }).collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_group));

    let list = List::new(items)
        .block(
            Block::bordered()
                .title(" Groupes ")
                .border_style(border_style)
        )
        .highlight_style(Style::new().bg(Color::Blue).fg(Color::White).bold())
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut state);
}

// ── Panel Entrées ─────────────────────────────────────────────
fn render_entries(frame: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == FocusedPanel::Entries;
    let border_style = if focused {
        Style::new().fg(Color::Blue)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let items: Vec<ListItem> = app.entries.iter().map(|e| {
        let strength_indicator = match e.strength {
            PasswordStrength::Strong => Span::styled("●", Style::new().fg(Color::Green)),
            PasswordStrength::Medium => Span::styled("●", Style::new().fg(Color::Yellow)),
            PasswordStrength::Weak   => Span::styled("●", Style::new().fg(Color::Red)),
        };
        ListItem::new(Line::from(vec![
            Span::raw(format!("{:<22}", &e.title)),
            strength_indicator,
        ]))
    }).collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_entry));

    let list = List::new(items)
        .block(
            Block::bordered()
                .title(" Entrées ")
                .border_style(border_style)
        )
        .highlight_style(Style::new().bg(Color::Blue).fg(Color::White).bold())
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut state);
}

// ── Panel Détail ──────────────────────────────────────────────
fn render_detail(frame: &mut Frame, app: &App, area: Rect) {
    let focused = app.focus == FocusedPanel::Detail;
    let border_style = if focused {
        Style::new().fg(Color::Blue)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let entry = app.selected_entry();

    let strength_text = match entry.strength {
        PasswordStrength::Strong => Span::styled("● Forte",  Style::new().fg(Color::Green)),
        PasswordStrength::Medium => Span::styled("● Moyenne",Style::new().fg(Color::Yellow)),
        PasswordStrength::Weak   => Span::styled("● Faible", Style::new().fg(Color::Red)),
    };

    let text = vec![
        Line::from(Span::styled(&entry.title, Style::new().bold().fg(Color::White))),
        Line::from(Span::styled(&entry.url, Style::new().fg(Color::Blue))),
        Line::from(""),
        Line::from(vec![
            Span::styled("LOGIN   ", Style::new().fg(Color::DarkGray)),
            Span::raw(&entry.username),
        ]),
        Line::from(vec![
            Span::styled("PASSWD  ", Style::new().fg(Color::DarkGray)),
            Span::raw("••••••••••  "),
            strength_text,
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("MODIFIÉ ", Style::new().fg(Color::DarkGray)),
            Span::raw(&entry.last_modified),
        ]),
        Line::from(vec![
            Span::styled("TAGS    ", Style::new().fg(Color::DarkGray)),
            Span::raw(entry.tags.join(", ")),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::bordered()
                .title(" Détail ")
                .border_style(border_style)
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

// ── Barre de raccourcis ───────────────────────────────────────
fn render_keybinds(frame: &mut Frame, area: Rect) {
    let text = Paragraph::new(Line::from(vec![
        key_hint("Tab", "panel"),
        Span::raw("  "),
        key_hint("j/k", "naviguer"),
        Span::raw("  "),
        key_hint("n", "nouveau"),
        Span::raw("  "),
        key_hint("e", "éditer"),
        Span::raw("  "),
        key_hint("y", "copier login"),
        Span::raw("  "),
        key_hint("p", "copier mdp"),
        Span::raw("  "),
        key_hint("q", "quitter"),
    ]))
        .style(Style::new().bg(Color::DarkGray));
    frame.render_widget(text, area);
}

fn key_hint(key: &str, desc: &str) -> Span<'static> {
    // On concatène en owned String pour éviter les problèmes de lifetime
    let s = format!(" {} {}", key, desc);
    Span::styled(s, Style::new().fg(Color::White))
}