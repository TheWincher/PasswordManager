pub mod actions;
pub mod audit;
pub mod state;

use std::path::PathBuf;

pub use audit::AuditCategory;
pub use state::*;

use audit::{audit_score, entries_for_category};

use crate::vault;

pub struct App {
    pub groups: Vec<Group>,
    pub selected_group: usize,
    pub entries: Vec<Entry>,
    pub selected_entry: usize,
    pub filtered_indices: Vec<usize>,
    pub focus: FocusedPanel,
    pub mode: AppMode,
    pub form: NewEntryForm,
    pub search_query: String,
    pub audit_category: usize,
    pub audit_entry: usize,
    pub audit_focus: AuditFocus,
    pub master_password: String,
    pub unlock_error: bool,
    pub vault_path: PathBuf,
    pub clipboard_msg: Option<String>,
    pub new_group_input: String,
}

impl App {
    pub fn new() -> Self {
        let entries = vec![];
        let filtered_indices = vec![];

        Self {
            groups: vec![
                Group {
                    name: "Tous".into(),
                    icon: "★",
                },
                Group {
                    name: "Web".into(),
                    icon: "🌐",
                },
                Group {
                    name: "Finance".into(),
                    icon: "💳",
                },
                Group {
                    name: "Dev".into(),
                    icon: "🔧",
                },
                Group {
                    name: "Email".into(),
                    icon: "📧",
                },
            ],
            selected_group: 0,
            entries,
            selected_entry: 0,
            filtered_indices,
            focus: FocusedPanel::Entries,
            mode: AppMode::Locked,
            form: NewEntryForm::new(),
            search_query: String::new(),
            audit_category: 0,
            audit_entry: 0,
            audit_focus: AuditFocus::Categories,
            master_password: String::new(),
            new_group_input: String::new(),
            unlock_error: false,
            vault_path: dirs::home_dir()
                .unwrap()
                .join(".password_manager/vault.json"),
            clipboard_msg: None,
        }
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) {
        actions::handle_key(self, key);
    }

    pub fn selected_entry(&self) -> Option<&Entry> {
        self.filtered_indices
            .get(self.selected_entry)
            .map(|&i| &self.entries[i])
    }

    pub fn visible_entries(&self) -> Vec<&Entry> {
        self.filtered_indices
            .iter()
            .map(|&i| &self.entries[i])
            .collect()
    }

    pub fn audit_score(&self) -> u8 {
        audit_score(&self.entries)
    }

    pub fn entries_for_category(&self, cat: &AuditCategory) -> Vec<&Entry> {
        entries_for_category(&self.entries, cat)
    }

    pub fn save(&self) -> Result<(), vault::VaultError> {
        vault::save(
            &vault::VaultData {
                groups: self.groups.clone(),
                entries: self.entries.clone(),
            },
            &self.master_password,
            &self.vault_path,
        )
    }
}
