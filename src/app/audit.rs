use crate::app::state::{Entry, PasswordStrength};

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
        .filter(|e| match cat {
            AuditCategory::Weak => e.strength == PasswordStrength::Weak,
            AuditCategory::Reused => e.is_reused,
            AuditCategory::Old => e.is_old,
            AuditCategory::NoTwoFactor => !e.two_factor,
            AuditCategory::Ok => {
                e.strength != PasswordStrength::Weak && !e.is_reused && !e.is_old && e.two_factor
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
            e.strength != PasswordStrength::Weak && !e.is_reused && !e.is_old && e.two_factor
        })
        .count() as f32;
    ((ok / total) * 100.0) as u8
}
