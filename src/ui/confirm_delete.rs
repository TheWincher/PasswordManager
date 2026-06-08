use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph},
};

use crate::{app::App, ui::utils::centered_rect};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let Some(entry) = app.selected_entry() else {
        return;
    };

    // Centrage : 50% de largeur, hauteur fixe
    let popup_area = centered_rect(50, 3, area);

    // On efface la zone derrière (effet overlay)
    frame.render_widget(Clear, popup_area);

    let block = Block::bordered()
        .border_style(Style::new().fg(Color::Blue))
        .style(Style::new().bg(Color::Reset));

    let inner = block.inner(popup_area);

    let text = Paragraph::new(format!(" Supprimer \"{}\" ? [y]es / [n]o", entry.title)).centered();

    frame.render_widget(block, popup_area);
    frame.render_widget(text, inner);
}
