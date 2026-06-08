use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Clear, Paragraph},
};

use crate::{
    app::{App, NewEntryForm},
    ui::utils::centered_rect,
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    // Centrage : 50% de largeur, hauteur fixe
    let popup_area = centered_rect(50, 22, area);

    // On efface la zone derrière (effet overlay)
    frame.render_widget(Clear, popup_area);

    let title = if app.form.editing_index.is_some() {
        " ✎ Éditer l'entrée "
    } else {
        " ✚ Nouvelle entrée "
    };

    let block = Block::bordered()
        .title(title)
        .border_style(Style::new().fg(Color::Blue))
        .style(Style::new().bg(Color::Reset));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    // Layout vertical : un bloc par champ
    let field_areas = Layout::vertical(
        std::iter::repeat(Constraint::Length(3))
            .take(4)
            .collect::<Vec<_>>(),
    )
    .split(inner);

    let names = NewEntryForm::field_names();

    for (i, (name, field_area)) in names.iter().zip(field_areas.iter()).enumerate() {
        let is_active = i == app.form.focused_field;

        let border_style = if is_active {
            Style::new().fg(Color::Blue)
        } else {
            Style::new().fg(Color::DarkGray)
        };

        // Pour le mot de passe, masque la valeur
        let display_value = if i == 3 {
            "•".repeat(app.form.fields[i].len())
        } else {
            app.form.fields[i].clone()
        };

        // Curseur clignotant sur le champ actif
        let content = if is_active {
            format!("{}_", display_value) // '_' simule le curseur
        } else {
            display_value
        };

        let widget = Paragraph::new(content).block(
            Block::bordered()
                .title(format!(" {} ", name))
                .border_style(border_style),
        );

        frame.render_widget(widget, *field_area);
    }
}
