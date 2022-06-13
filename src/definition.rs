#[derive(Debug, Default, Clone)]
pub struct Definition {
    pub writing_method: String,
    pub writing_method_two: Option<String>,
    pub prounciation: String,
    pub english: String,
    pub count: i64
}

pub trait CommonDefinitionLanguage {
    fn get_english_translations(&self) -> Vec<String>;
}