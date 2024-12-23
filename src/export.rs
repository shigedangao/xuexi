use crate::error::DictionaryError;
use csv::Writer;
use serde::{Serialize, Serializer};

pub trait Export {
    /// Export a a type to CSV
    ///     - Definitions: In this case this will return a csv of definitions
    fn to_csv(&self) -> Result<String, DictionaryError>;
}

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

// Utility method to convert a vec into a string for the csv library
pub fn serialize_vec_to_string<S, T>(items: &[T], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: AsRef<str>,
{
    let str_vec: Vec<String> = items.iter().map(|s| s.as_ref().to_string()).collect();
    let str = str_vec.join(",");

    s.serialize_str(&str)
}
