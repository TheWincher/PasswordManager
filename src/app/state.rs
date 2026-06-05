#[derive(Clone, PartialEq)]
pub enum PasswordStrength {
    Strong,
    Medium,
    Weak,
}

#[derive(Clone)]
pub struct Entry {
    pub title: String,
    pub username: String,
    pub url: String,
    pub strength: PasswordStrength,
    pub tags: Vec<String>,
    pub last_modified: String,
    pub two_factor: bool,
    pub is_old: bool,
    pub is_reused: bool,
}

#[derive(Clone, PartialEq)]
pub struct Group {
    pub name: String,
    pub icon: &'static str,
}

#[derive(PartialEq)]
pub enum AppMode {
    Locked,
    Normal,
    Popup,
    Search,
    Audit,
}

#[derive(PartialEq)]
pub enum FocusedPanel {
    Groups,
    Entries,
    Detail,
}

#[derive(PartialEq)]
pub enum AuditFocus {
    Categories,
    Entries,
}

pub struct NewEntryForm {
    pub fields: [String; 4],
    pub focused_field: usize,
}

impl NewEntryForm {
    pub fn new() -> Self {
        Self {
            fields: [String::new(), String::new(), String::new(), String::new()],
            focused_field: 0,
        }
    }
    pub fn field_names() -> [&'static str; 4] {
        ["Titre", "URL", "Identifiant", "Mot de passe"]
    }
}
