use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::app::state::Entry;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum PasswordStrength {
    Strong,
    Medium,
    Weak,
}

#[derive(PartialEq, Clone)]
pub enum AuditCategory {
    Weak,
    Reused,
    Old,
    NoTwoFactor,
    Ok,
}

impl AuditCategory {
    pub fn label(&self) -> &str {
        match self {
            AuditCategory::Weak => "Mots de passe faibles",
            AuditCategory::Reused => "Réutilisés",
            AuditCategory::Old => "Anciens (+1 an)",
            AuditCategory::NoTwoFactor => "Sans 2FA",
            AuditCategory::Ok => "Tout OK",
        }
    }
    pub fn icon(&self) -> &str {
        match self {
            AuditCategory::Weak => "⚠",
            AuditCategory::Reused => "♻",
            AuditCategory::Old => "🕐",
            AuditCategory::NoTwoFactor => "🔐",
            AuditCategory::Ok => "✓",
        }
    }
    pub fn all() -> Vec<AuditCategory> {
        vec![
            AuditCategory::Weak,
            AuditCategory::Reused,
            AuditCategory::Old,
            AuditCategory::NoTwoFactor,
            AuditCategory::Ok,
        ]
    }
}

pub fn entries_for_category<'a>(entries: &'a [Entry], cat: &AuditCategory) -> Vec<&'a Entry> {
    entries
        .iter()
        .filter(|e| {
            let password_strength = password_strength(&e.password);
            let is_reused = is_reused(e, entries);
            let is_old = is_old(&e.last_modified);

            match cat {
                AuditCategory::Weak => password_strength == PasswordStrength::Weak,
                AuditCategory::Reused => is_reused,
                AuditCategory::Old => is_old,
                AuditCategory::NoTwoFactor => !e.two_factor,
                AuditCategory::Ok => {
                    password_strength != PasswordStrength::Weak
                        && !is_reused
                        && !is_old
                        && e.two_factor
                }
            }
        })
        .collect()
}

pub fn audit_score(entries: &[Entry]) -> u8 {
    let total = entries.len() as f32;
    if total == 0.0 {
        return 100;
    }
    let ok = entries
        .iter()
        .filter(|e| {
            password_strength(&e.password) != PasswordStrength::Weak
                && !is_reused(e, entries)
                && !is_old(&e.last_modified)
                && e.two_factor
        })
        .count() as f32;
    ((ok / total) * 100.0) as u8
}

pub fn password_strength(password: &str) -> PasswordStrength {
    let mut score = 0;

    if password.len() >= 8 {
        score += 1;
    }

    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_uppercase()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_numeric()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_ascii_punctuation()) {
        score += 1;
    }

    if score >= 4 {
        PasswordStrength::Strong
    } else if score >= 2 {
        PasswordStrength::Medium
    } else {
        PasswordStrength::Weak
    }
}

fn is_old(last_modified: &str) -> bool {
    let Ok(date) = NaiveDate::parse_from_str(last_modified, "%Y-%m-%d") else {
        return false;
    };

    chrono::Local::now()
        .date_naive()
        .signed_duration_since(date)
        .num_days()
        > 365
}

fn is_reused(entry: &Entry, all_entries: &[Entry]) -> bool {
    all_entries
        .iter()
        .filter(|e| !std::ptr::eq(*e, entry))
        .any(|e| e.password == entry.password)
}
