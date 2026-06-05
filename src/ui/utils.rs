use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
};

pub fn key_hint(key: &str, desc: &str) -> Span<'static> {
    // On concatène en owned String pour éviter les problèmes de lifetime
    let s = format!(" {} {}", key, desc);
    Span::styled(s, Style::new().fg(Color::White))
}

pub fn centered_rect(percent_x: u16, height: u16, area: Rect) -> Rect {
    let w = area.width * percent_x / 100;
    let x = (area.width - w) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, w, height.min(area.height))
}
