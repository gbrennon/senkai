use std::collections::BTreeMap;

use crate::error::{Error, Result};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    values: BTreeMap<String, String>,
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_values<I, K, V>(values: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        Self {
            values: values
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn names(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(String::as_str)
    }

    pub fn set(&mut self, name: &str, value: &str) -> Result<()> {
        let name = name.trim();
        if name.is_empty() {
            return Err(Error::new("Setting name cannot be empty"));
        }
        self.values.insert(name.to_string(), value.to_string());
        Ok(())
    }

    pub fn with_value(mut self, name: &str, value: &str) -> Result<Self> {
        self.set(name, value)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_settings_are_empty() {
        assert!(Settings::new().is_empty());
    }

    #[test]
    fn setting_names_are_sorted() {
        let settings = Settings::from_values([("theme", "dark"), ("editor", "vim")]);

        assert_eq!(settings.names(), vec!["editor", "theme"]);
    }

    #[test]
    fn setting_value_can_be_read() {
        let settings = Settings::from_values([("theme", "dark")]);

        assert_eq!(settings.get("theme"), Some("dark"));
        assert_eq!(settings.get("missing"), None);
    }

    #[test]
    fn setting_value_can_be_replaced() {
        let mut settings = Settings::new();

        settings.set("theme", "dark").unwrap();
        settings.set("theme", "light").unwrap();

        assert_eq!(settings.get("theme"), Some("light"));
    }

    #[test]
    fn empty_setting_name_is_rejected() {
        let result = Settings::new().with_value("", "dark");

        assert!(result.is_err());
    }
}
