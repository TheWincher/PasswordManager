use crossterm::event::KeyCode;

// ── Données simulées ──────────────────────────────────────────
#[derive(Clone)]
pub struct Entry {
    pub title: String,
    pub username: String,
    pub url: String,
    pub strength: PasswordStrength,
    pub tags: Vec<String>,
    pub last_modified: String,
}

#[derive(Clone, PartialEq)]
pub enum PasswordStrength {
    Strong,
    Medium,
    Weak,
}

#[derive(Clone, PartialEq)]
pub struct Group {
    pub name: String,
    pub icon: &'static str,
}

// ── Quel panel a le focus ─────────────────────────────────────
#[derive(PartialEq)]
pub enum FocusedPanel {
    Groups,
    Entries,
    Detail,
}

pub struct App {
    pub groups: Vec<Group>,
    pub selected_group: usize,

    pub entries: Vec<Entry>,
    pub selected_entry: usize,

    pub focus: FocusedPanel,
}

impl App {
    pub fn new() -> Self {
        let groups = vec![
            Group { name: "Web".into(),     icon: "🌐" },
            Group { name: "Finance".into(), icon: "💳" },
            Group { name: "Dev".into(),     icon: "🔧" },
            Group { name: "Email".into(),   icon: "📧" },
        ];

        let entries = vec![
            Entry {
                title: "GitHub".into(),
                username: "jean@example.com".into(),
                url: "https://github.com".into(),
                strength: PasswordStrength::Strong,
                tags: vec!["dev".into(), "perso".into()],
                last_modified: "2024-11-01".into(),
            },
            Entry {
                title: "Twitter / X".into(),
                username: "jean@example.com".into(),
                url: "https://twitter.com".into(),
                strength: PasswordStrength::Weak,
                tags: vec!["social".into()],
                last_modified: "2021-03-14".into(),
            },
            Entry {
                title: "LinkedIn".into(),
                username: "jean.dupont@gmail.com".into(),
                url: "https://linkedin.com".into(),
                strength: PasswordStrength::Strong,
                tags: vec!["pro".into()],
                last_modified: "2024-06-20".into(),
            },
        ];

        Self {
            groups,
            selected_group: 0,
            entries,
            selected_entry: 0,
            focus: FocusedPanel::Entries,
        }
    }
    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => self.cycle_focus(),
            KeyCode::Up | KeyCode::Char('k') => self.move_up(),
            KeyCode::Down | KeyCode::Char('j') => self.move_down(),
            _ => {}
        }
    }

    fn cycle_focus(&mut self) {
        self.focus = match self.focus {
            FocusedPanel::Groups  => FocusedPanel::Entries,
            FocusedPanel::Entries => FocusedPanel::Detail,
            FocusedPanel::Detail  => FocusedPanel::Groups,
        };
    }

    fn move_up(&mut self) {
        match self.focus {
            FocusedPanel::Groups  => { if self.selected_group > 0 { self.selected_group -= 1; } }
            FocusedPanel::Entries => { if self.selected_entry > 0 { self.selected_entry -= 1; } }
            _ => {}
        }
    }

    fn move_down(&mut self) {
        match self.focus {
            FocusedPanel::Groups  => { if self.selected_group < self.groups.len() - 1 { self.selected_group += 1; } }
            FocusedPanel::Entries => { if self.selected_entry < self.entries.len() - 1 { self.selected_entry += 1; } }
            _ => {}
        }
    }

    pub fn selected_entry(&self) -> &Entry {
        &self.entries[self.selected_entry]
    }
}