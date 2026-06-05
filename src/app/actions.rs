use crate::app::audit::AuditCategory;
use crate::app::state::*;
use crate::clipboard;
use crate::vault::{self, VaultError};
use crossterm::event::KeyCode;

pub fn handle_key(app: &mut crate::app::App, key: KeyCode) {
    match app.mode {
        AppMode::Locked => handle_locked(app, key),
        AppMode::Normal => handle_normal(app, key),
        AppMode::Popup => handle_popup(app, key),
        AppMode::Search => handle_search(app, key),
        AppMode::Audit => handle_audit(app, key),
    }
}

fn handle_locked(app: &mut crate::app::App, key: KeyCode) {
    match key {
        KeyCode::Enter => {
            let load_result = vault::load(&app.master_password, &app.vault_path);
            match load_result {
                Ok(entries) => {
                    app.mode = AppMode::Normal;
                    app.master_password.clear();
                    app.unlock_error = false;
                    app.entries = entries;
                    update_filter(app);
                }
                Err(VaultError::Io(_)) => {
                    // premier lancement, vault n'existe pas encore
                    app.mode = AppMode::Normal;
                    app.entries = vec![];
                    app.unlock_error = false;
                }
                Err(_) => {
                    // mauvais mot de passe ou fichier corrompu
                    app.unlock_error = true;
                    app.master_password.clear();
                }
            }
        }
        KeyCode::Backspace => {
            app.master_password.pop();
            app.unlock_error = false;
        }
        KeyCode::Char(c) => {
            app.master_password.push(c);
            app.unlock_error = false;
        }
        _ => {}
    }
}

fn handle_normal(app: &mut crate::app::App, key: KeyCode) {
    app.clipboard_msg = None;
    match key {
        KeyCode::Tab => cycle_focus(app),
        KeyCode::Up | KeyCode::Char('k') => move_up(app),
        KeyCode::Down | KeyCode::Char('j') => move_down(app),
        KeyCode::Char('n') => {
            app.mode = AppMode::Popup;
            app.form = NewEntryForm::new();
        }
        KeyCode::Char('d') => {
            if app.selected_entry().is_some() {
                app.entries.remove(app.filtered_indices[app.selected_entry]);
                update_filter(app);
                match app.save() {
                    Ok(_) => app.clipboard_msg = Some("✓ Mot de passe supprimé".to_string()),
                    Err(_) => {
                        app.clipboard_msg = Some("✗ Erreur lors de la suppréssion".to_string())
                    }
                };

                // Cas où on supprime la dernière entrée de la liste
                if app.selected_entry >= app.filtered_indices.len() && app.selected_entry > 0 {
                    app.selected_entry -= 1;
                }
            }
        }
        KeyCode::Char('/') => {
            app.mode = AppMode::Search;
            app.search_query.clear();
            update_filter(app);
        }
        KeyCode::Char('a') => {
            app.mode = AppMode::Audit;
            app.audit_category = 0;
            app.audit_entry = 0;
            app.audit_focus = AuditFocus::Categories;
        }
        KeyCode::Char('l') => {
            app.mode = AppMode::Locked;
            app.master_password.clear();
            app.unlock_error = false;
        }
        KeyCode::Char('y') => {
            if let Some(entry) = app.selected_entry() {
                app.clipboard_msg = match clipboard::copy(&entry.username) {
                    Ok(_) => Some("✓ Identifiant copié".to_string()),
                    Err(_) => Some("✗ Erreur de copie".to_string()),
                };
            }
        }
        KeyCode::Char('p') => {
            if let Some(entry) = app.selected_entry() {
                app.clipboard_msg = match clipboard::copy(&entry.password) {
                    Ok(_) => Some("✓ Mot de passe copié".to_string()),
                    Err(_) => Some("✗ Erreur de copie".to_string()),
                };
            }
        }
        _ => {}
    }
}

fn handle_popup(app: &mut crate::app::App, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            app.mode = AppMode::Normal;
        }
        KeyCode::Tab => {
            app.form.focused_field = (app.form.focused_field + 1) % 4;
        }
        KeyCode::BackTab => {
            if app.form.focused_field == 0 {
                app.form.focused_field = 3;
            } else {
                app.form.focused_field -= 1;
            }
        }
        KeyCode::Enter => {
            let f = &app.form.fields;
            if !f[0].is_empty() {
                app.entries.push(Entry {
                    title: f[0].clone(),
                    url: f[1].clone(),
                    username: f[2].clone(),
                    strength: PasswordStrength::Medium,
                    tags: vec![],
                    last_modified: "2026-06-04".into(),
                    two_factor: false,
                    is_old: false,
                    is_reused: false,
                    password: f[3].clone(),
                });
                app.selected_entry = app.entries.len() - 1;
                match app.save() {
                    Ok(_) => app.clipboard_msg = Some("✓ Mot de passe ajouté".to_string()),
                    Err(_) => app.clipboard_msg = Some("✗ Erreur lors de l'ajout".to_string()),
                };

                update_filter(app);
            }
            app.mode = AppMode::Normal;
        }
        KeyCode::Backspace => {
            app.form.fields[app.form.focused_field].pop();
        }
        KeyCode::Char(c) => {
            app.form.fields[app.form.focused_field].push(c);
        }
        _ => {}
    }
}

fn handle_search(app: &mut crate::app::App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Enter => {
            app.mode = AppMode::Normal;
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            update_filter(app);
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            update_filter(app);
        }
        _ => {}
    }
}

fn handle_audit(app: &mut crate::app::App, key: KeyCode) {
    let cat_count = AuditCategory::all().len();
    match key {
        KeyCode::Esc => {
            app.mode = AppMode::Normal;
        }
        KeyCode::Tab => {
            app.audit_focus = match app.audit_focus {
                AuditFocus::Categories => AuditFocus::Entries,
                AuditFocus::Entries => AuditFocus::Categories,
            };
        }
        KeyCode::Up | KeyCode::Char('k') => match app.audit_focus {
            AuditFocus::Categories => {
                if app.audit_category > 0 {
                    app.audit_category -= 1;
                    app.audit_entry = 0;
                }
            }
            AuditFocus::Entries => {
                if app.audit_entry > 0 {
                    app.audit_entry -= 1;
                }
            }
        },
        KeyCode::Down | KeyCode::Char('j') => match app.audit_focus {
            AuditFocus::Categories => {
                if app.audit_category < cat_count - 1 {
                    app.audit_category += 1;
                    app.audit_entry = 0;
                }
            }
            AuditFocus::Entries => {
                let cats = AuditCategory::all();
                let count = crate::app::audit::entries_for_category(
                    &app.entries,
                    &cats[app.audit_category],
                )
                .len();
                if app.audit_entry < count.saturating_sub(1) {
                    app.audit_entry += 1;
                }
            }
        },
        _ => {}
    }
}

// ── Helpers ───────────────────────────────────────────────────

fn cycle_focus(app: &mut crate::app::App) {
    app.focus = match app.focus {
        FocusedPanel::Groups => FocusedPanel::Entries,
        FocusedPanel::Entries => FocusedPanel::Detail,
        FocusedPanel::Detail => FocusedPanel::Groups,
    };
}

fn move_up(app: &mut crate::app::App) {
    match app.focus {
        FocusedPanel::Groups => {
            if app.selected_group > 0 {
                app.selected_group -= 1;
            }
        }
        FocusedPanel::Entries => {
            if app.selected_entry > 0 {
                app.selected_entry -= 1;
            }
        }
        _ => {}
    }
}

fn move_down(app: &mut crate::app::App) {
    match app.focus {
        FocusedPanel::Groups => {
            if app.selected_group < app.groups.len() - 1 {
                app.selected_group += 1;
            }
        }
        FocusedPanel::Entries => {
            if app.selected_entry < app.filtered_indices.len().saturating_sub(1) {
                app.selected_entry += 1;
            }
        }
        _ => {}
    }
}

pub fn update_filter(app: &mut crate::app::App) {
    let q = app.search_query.to_lowercase();
    app.filtered_indices = app
        .entries
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            q.is_empty()
                || e.title.to_lowercase().contains(&q)
                || e.url.to_lowercase().contains(&q)
                || e.username.to_lowercase().contains(&q)
                || e.tags.iter().any(|t| t.to_lowercase().contains(&q))
        })
        .map(|(i, _)| i)
        .collect();

    if app.selected_entry >= app.filtered_indices.len() {
        app.selected_entry = 0;
    }
}
