use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, ListState, Paragraph},
};

use crate::app::{App, AuditCategory, AuditFocus};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::horizontal([
        Constraint::Length(28), // catégories + score
        Constraint::Min(0),     // entrées de la catégorie
    ])
    .split(area);

    render_audit_categories(frame, app, cols[0]);
    render_audit_entries(frame, app, cols[1]);
}

fn render_audit_categories(frame: &mut Frame, app: &App, area: Rect) {
    let rows = Layout::vertical([
        Constraint::Min(0),    // liste catégories
        Constraint::Length(5), // score
    ])
    .split(area);

    // ── Liste des catégories ──
    let focused = app.audit_focus == AuditFocus::Categories;
    let border_style = if focused {
        Style::new().fg(Color::Blue)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let cats = AuditCategory::all();
    let items: Vec<ListItem> = cats
        .iter()
        .map(|cat| {
            let count = app.entries_for_category(cat).len();
            let (icon_style, count_style) = match cat {
                AuditCategory::Ok => (Style::new().fg(Color::Green), Style::new().fg(Color::Green)),
                _ => (
                    Style::new().fg(Color::Yellow),
                    Style::new().fg(Color::Yellow),
                ),
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", cat.icon()), icon_style),
                Span::raw(format!("{:<20}", cat.label())),
                Span::styled(format!("{:>2}", count), count_style),
            ]))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.audit_category));

    let list = List::new(items)
        .block(
            Block::bordered()
                .title(" 🛡 Audit ")
                .border_style(border_style),
        )
        .highlight_style(Style::new().bg(Color::Blue).fg(Color::White).bold())
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, rows[0], &mut state);

    // ── Score ──
    let score = app.audit_score();
    let (score_color, score_label) = match score {
        80..=100 => (Color::Green, "Excellent"),
        50..=79 => (Color::Yellow, "Améliorable"),
        _ => (Color::Red, "Critique"),
    };

    let bar_filled = (score as usize * 18) / 100;
    let bar = format!(
        "[{}{}]",
        "█".repeat(bar_filled),
        "░".repeat(18 - bar_filled)
    );

    let score_text = vec![
        Line::from(vec![
            Span::styled(format!("{:>3}", score), Style::new().fg(score_color).bold()),
            Span::styled("/100", Style::new().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(score_label, Style::new().fg(score_color)),
        ]),
        Line::from(Span::styled(bar, Style::new().fg(score_color))),
    ];

    let score_widget = Paragraph::new(score_text).block(
        Block::bordered()
            .title(" Score ")
            .border_style(Style::new().fg(Color::DarkGray)),
    );
    frame.render_widget(score_widget, rows[1]);
}

fn render_audit_entries(frame: &mut Frame, app: &App, area: Rect) {
    let focused = app.audit_focus == AuditFocus::Entries;
    let border_style = if focused {
        Style::new().fg(Color::Blue)
    } else {
        Style::new().fg(Color::DarkGray)
    };

    let cats = AuditCategory::all();
    let cat = &cats[app.audit_category];
    let entries = app.entries_for_category(cat);

    let title = format!(" {} {} ", cat.icon(), cat.label());

    if entries.is_empty() {
        let empty = Paragraph::new("\n  ✓ Aucun problème dans cette catégorie.")
            .style(Style::new().fg(Color::Green))
            .block(Block::bordered().title(title).border_style(border_style));
        frame.render_widget(empty, area);
        return;
    }

    let items: Vec<ListItem> = entries
        .iter()
        .map(|e| {
            let detail = match cat {
                AuditCategory::Weak => format!("entropie faible · {}", e.last_modified),
                AuditCategory::Old => format!("modifié le {}", e.last_modified),
                AuditCategory::Reused => "mot de passe réutilisé".into(),
                AuditCategory::NoTwoFactor => "2FA non configuré".into(),
                AuditCategory::Ok => "aucun problème".into(),
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{:<22}", e.title), Style::new().fg(Color::White)),
                Span::styled(detail, Style::new().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.audit_entry));

    let list = List::new(items)
        .block(Block::bordered().title(title).border_style(border_style))
        .highlight_style(Style::new().bg(Color::Blue).fg(Color::White).bold())
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut state);
}
