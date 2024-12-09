use std::{collections::HashMap, path::Path};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// English should always be embedded in the binary for the logger to work,
//   translations should be loaded via the resource loader system,
//   translations should be overridable by the user via the resource loader system,
//   mods should be able to provide custom translation keys which do not override the internal keys if they exist

/// Structure containing translations
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct I18N {
    /// `HashMap`<`Locale`, `HashMap`<`TranslationKey`, `TranslationValue`>>
    trs: HashMap<String, HashMap<String, String>>,
}

impl I18N {
    /// Create's new I18N Struct
    #[must_use]
    pub fn new(_locales_path: &Path) -> Self {
        // crate::resources::get_resource_string(locales_path);
        // let _locales_contents: String = fs::read_to_string(locales_path).unwrap();

        // Populate translation struct
        // let trs: HashMap<String, HashMap<String, String>> = toml::from_str(locales_contents.as_str()).unwrap();
        let trs: HashMap<String, HashMap<String, String>> =
            HashMap::<String, HashMap<String, String>>::new();

        Self { trs }
    }

    /// Get available locales
    #[must_use]
    pub fn available_locales(&self) -> Vec<&str> {
        // self.trs.keys().map(std::string::String::as_str(k)).collect();
        self.trs.keys().map(|k| k.as_str()).collect()
    }

    /// Get translated string
    #[must_use]
    pub fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        self.trs.get(locale)?.get(key).map(|k| k.as_str())
    }
}
