// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-module-blacklist: module blacklist component model.
//!
//! Real implementation: blacklists, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blacklist {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleBlacklistError {
    EmptyBlacklistName,
    DuplicateBlacklist(String),
    EmptyEventSubject,
}

impl fmt::Display for ModuleBlacklistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBlacklistName => write!(f, "blacklist name must not be empty"),
            Self::DuplicateBlacklist(id) => write!(f, "duplicate blacklist {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ModuleBlacklistError {}

pub struct ModuleBlacklist {
    pub name: String,
    blacklists: BTreeMap<String, Blacklist>,
    events: BTreeMap<String, Event>,
}

impl ModuleBlacklist {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            blacklists: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_blacklist(&mut self, item: Blacklist) -> Result<(), ModuleBlacklistError> {
        if item.name.trim().is_empty() {
            return Err(ModuleBlacklistError::EmptyBlacklistName);
        }
        let id = item.id.clone();
        if self.blacklists.contains_key(&id) {
            return Err(ModuleBlacklistError::DuplicateBlacklist(id.clone()));
        }
        self.blacklists.insert(id, item);
        Ok(())
    }

    pub fn blacklist_count(&self) -> usize {
        self.blacklists.len()
    }

    pub fn get_blacklist(&self, id: &str) -> Option<&Blacklist> {
        self.blacklists.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ModuleBlacklistError> {
        if event.subject.trim().is_empty() {
            return Err(ModuleBlacklistError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_blacklist(id: &str, name: &str) -> Blacklist {
        Blacklist {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ModuleBlacklist::new("ModuleBlacklist");
        assert_eq!(m.name, "ModuleBlacklist");
        assert_eq!(m.blacklist_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_blacklist() {
        let mut m = ModuleBlacklist::new("ModuleBlacklist");
        m.add_blacklist(sample_blacklist("a1", "Alpha")).unwrap();
        assert_eq!(m.blacklist_count(), 1);
        assert!(m.get_blacklist("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ModuleBlacklist::new("ModuleBlacklist");
        assert!(m.add_blacklist(sample_blacklist("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ModuleBlacklist::new("ModuleBlacklist");
        m.add_blacklist(sample_blacklist("x", "One")).unwrap();
        assert!(m.add_blacklist(sample_blacklist("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ModuleBlacklist::new("ModuleBlacklist");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
