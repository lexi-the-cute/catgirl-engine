use std::collections::HashMap;
use std::fs;

use rust_i18n::Backend;

/// Structure containing translations
pub struct I18N {
    trs: HashMap<String, HashMap<String, String>>,
}

impl I18N {
    /// Create's new I18N Struct
    pub fn new(locales_path: &str) -> Self {
        let _locales_contents: String = fs::read_to_string(locales_path).unwrap();
        // let trs: HashMap<String, HashMap<String, String>> = toml::from_str(locales_contents.as_str()).unwrap();
        let trs: HashMap<String, HashMap<String, String>> =
            HashMap::<String, HashMap<String, String>>::new();

        return Self { trs };
    }
}

impl Backend for I18N {
    fn available_locales(&self) -> Vec<&str> {
        return self.trs.keys().map(|k| k.as_str()).collect();
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        // Write your own lookup logic here.
        // For example load from database
        return self.trs.get(locale)?.get(key).map(|k| k.as_str());
    }
}
