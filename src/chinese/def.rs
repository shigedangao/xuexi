use crate::definition::{CommonDefinitionLanguage, Definition};

impl CommonDefinitionLanguage for Definition {
    /// Get a vector of english translation from the string representation
    fn get_english_translations(&self) -> Vec<String> {
        self.english.split('/')
            .into_iter()
            .filter_map(|s| {
                if s.is_empty() {
                    return None;
                }

                Some(s.trim().to_string())
            })
            .collect::<Vec<String>>()
    }
}
