use std::collections::HashMap;
use std::fs;

// English should always be embedded in the binary for the logger to work,
//   translations should be loaded via the resource loader system,
//   translations should be overridable by the user via the resource loader system,
//   mods should be able to provide custom translation keys which do not override the internal keys if they exist

/// Structure containing translations
pub struct I18N {
    /// HashMap<Locale, HashMap<TranslationKey, TranslationValue>>
    trs: HashMap<String, HashMap<String, String>>,
}

impl I18N {
    /// Create's new I18N Struct
    pub fn new(locales_path: &str) -> Self {
        let _locales_contents: String = fs::read_to_string(locales_path).unwrap();

        // Populate translation struct
        // let trs: HashMap<String, HashMap<String, String>> = toml::from_str(locales_contents.as_str()).unwrap();
        let trs: HashMap<String, HashMap<String, String>> =
            HashMap::<String, HashMap<String, String>>::new();

        return Self { trs };
    }

    /// Get available locales
    pub fn available_locales(&self) -> Vec<&str> {
        return self.trs.keys().map(|k| k.as_str()).collect();
    }

    /// Get translated string
    pub fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        return self.trs.get(locale)?.get(key).map(|k| k.as_str());
    }
}
