pub mod actions;
pub mod audit;
pub mod state;

pub use audit::AuditCategory;
pub use state::*;

use audit::{audit_score, entries_for_category};

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
}

impl App {
    pub fn new() -> Self {
        let entries = vec![
            Entry {
                title: "GitHub".into(),
                username: "jean@example.com".into(),
                url: "https://github.com".into(),
                strength: PasswordStrength::Strong,
                tags: vec!["dev".into(), "perso".into()],
                last_modified: "2024-11-01".into(),
                two_factor: true,
                is_old: false,
                is_reused: false,
            },
            Entry {
                title: "Twitter / X".into(),
                username: "jean@example.com".into(),
                url: "https://twitter.com".into(),
                strength: PasswordStrength::Weak,
                tags: vec!["social".into()],
                last_modified: "2021-03-14".into(),
                two_factor: false,
                is_old: true,
                is_reused: true,
            },
            Entry {
                title: "LinkedIn".into(),
                username: "jean.dupont@gmail.com".into(),
                url: "https://linkedin.com".into(),
                strength: PasswordStrength::Strong,
                tags: vec!["pro".into()],
                last_modified: "2024-06-20".into(),
                two_factor: false,
                is_old: false,
                is_reused: false,
            },
            Entry {
                title: "BNP Paribas".into(),
                username: "jean.dupont@gmail.com".into(),
                url: "https://mabanque.bnpparibas.com".into(),
                strength: PasswordStrength::Weak,
                tags: vec!["finance".into()],
                last_modified: "2020-01-10".into(),
                two_factor: false,
                is_old: true,
                is_reused: true,
            },
        ];

        let filtered_indices = (0..entries.len()).collect();

        Self {
            groups: vec![
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
            unlock_error: false,
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
}
