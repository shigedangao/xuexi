use crate::error::DictionaryError;
use csv::Writer;
use serde::Serialize;

/// Helper method to export a vec of items which implement the Serialize trait
/// to a CSV string representation
///
/// # Arguments
///
/// * `items` - Vec<T>
pub fn export_to_csv<T: Serialize>(items: Vec<T>) -> Result<String, DictionaryError> {
    let mut wrt = Writer::from_writer(vec![]);
    for item in items {
        wrt.serialize(item)?;
    }

    let inner = wrt
        .into_inner()
        .map_err(|err| DictionaryError::Serialize(err.to_string()))?;

    let res = String::from_utf8(inner)?;

    Ok(res)
}

pub trait Export {
    /// Export a a type to CSV
    ///     - Definitions: In this case this will return a csv of definitions
    ///     - Character count: This is gonna return the character and the count amount
    fn to_csv(&self) -> Result<String, DictionaryError>;
}
