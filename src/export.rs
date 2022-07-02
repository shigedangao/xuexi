use csv::Writer;
use serde::Serialize;

use crate::error::LibError;

/// Helper method to export a vec of items which implement the Serialize trait
/// to a CSV string representation
/// 
/// # Arguments
/// 
/// * `items` - Vec<T>
pub fn export_to_csv<T: Serialize>(items: Vec<T>) -> Result<String, LibError> {
    let mut wrt = Writer::from_writer(vec![]);
    wrt.serialize(items)?;

    let inner = wrt.into_inner()
        .map_err(|err| LibError::Serialize(err.to_string()))?;

    let res = String::from_utf8(inner)?;

    Ok(res)
} 