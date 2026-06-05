pub mod audit;
pub mod keybinds;
pub mod locked;
pub mod panels;
pub mod popup;
pub mod searchbar;
pub mod statusbar;
mod utils;

use crate::app::{App, AppMode};
use ratatui::prelude::*;

pub fn render(frame: &mut Frame, app: &App) {
    if app.mode == AppMode::Locked {
        locked::render(frame, app, frame.area());
        return;
    }

    let area = frame.area();
    let show_search = app.mode == AppMode::Search || !app.search_query.is_empty();

    let root = if show_search {
        Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area)
    } else {
        Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area)
    };

    statusbar::render(frame, root[0], app);

    if show_search {
        searchbar::render(frame, root[1], app);
        render_main(frame, app, root[2]);
        keybinds::render(frame, root[3], app);
    } else {
        if app.mode == AppMode::Audit {
            audit::render(frame, app, root[1]);
        } else {
            render_main(frame, app, root[1]);
        }
        keybinds::render(frame, root[2], app);
    }

    if app.mode == AppMode::Popup {
        popup::render(frame, app, area);
    }
}

fn render_main(frame: &mut Frame, app: &App, area: Rect) {
    let cols = Layout::horizontal([
        Constraint::Length(20),
        Constraint::Min(25),
        Constraint::Length(38),
    ])
    .split(area);

    panels::groups::render(frame, app, cols[0]);
    panels::entries::render(frame, app, cols[1]);
    panels::detail::render(frame, app, cols[2]);
}
