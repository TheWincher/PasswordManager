use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
};

use crate::{
    app::App,
    ui::utils::{centered_rect, key_hint},
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    // Fond sombre
    let bg = Block::default().style(Style::new().bg(Color::Reset));
    frame.render_widget(bg, area);

    // Boîte centrée : 40% de large, 12 lignes de haut
    let popup_area = centered_rect(40, 12, area);
    frame.render_widget(Clear, popup_area);

    let block = Block::bordered()
        .title(" 🔒 passui ")
        .title_alignment(Alignment::Center)
        .border_style(Style::new().fg(Color::Blue));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Layout interne : icône + champ + erreur + hint
    let rows = Layout::vertical([
        Constraint::Length(2), // icône + nom du vault
        Constraint::Length(1), // séparateur
        Constraint::Length(3), // champ mot de passe
        Constraint::Length(1), // message d'erreur
        Constraint::Min(0),    // espace
        Constraint::Length(1), // hint
    ])
    .split(inner);

    // Vault name
    let vault_text = Paragraph::new(vec![
        Line::from(Span::styled(
            "personal.vault",
            Style::new().fg(Color::White).bold(),
        )),
        Line::from(Span::styled(
            "Entrez votre mot de passe maître",
            Style::new().fg(Color::DarkGray),
        )),
    ])
    .alignment(Alignment::Center);
    frame.render_widget(vault_text, rows[0]);

    // Champ mot de passe
    let masked = "•".repeat(app.master_password.len());
    let field_content = format!("{}_", masked);
    let field_style = if app.unlock_error {
        Style::new().fg(Color::Red)
    } else {
        Style::new().fg(Color::White)
    };
    let field = Paragraph::new(field_content)
        .style(field_style)
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .title(" Mot de passe maître ")
                .title_alignment(Alignment::Center)
                .border_style(if app.unlock_error {
                    Style::new().fg(Color::Red)
                } else {
                    Style::new().fg(Color::Blue)
                }),
        );
    frame.render_widget(field, rows[2]);

    // Message d'erreur
    if app.unlock_error {
        let err = Paragraph::new("✗ Mot de passe incorrect")
            .style(Style::new().fg(Color::Red))
            .alignment(Alignment::Center);
        frame.render_widget(err, rows[3]);
    }

    // Hint
    let hint = Paragraph::new(Line::from(vec![
        key_hint("↵", "déverrouiller"),
        Span::raw("   "),
        key_hint("q", "quitter"),
    ]))
    .alignment(Alignment::Center)
    .style(Style::new().fg(Color::DarkGray));
    frame.render_widget(hint, rows[5]);
}
