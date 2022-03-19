use crate::error::{Error, Result};
use std::{fmt::Debug, str::FromStr};

/// Parse a numeric value from an option and handle the error
pub(crate) fn parse_number<T>(value: Option<&str>, index: usize) -> Result<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let item = parse_str(value, index)?;
    let result = item.parse::<T>().map_err(|e| {
        Error::Parse(format!(
            "Error parsing '{}' in '{:?}': {:?}",
            item, value, e
        ))
    })?;

    Ok(result)
}

/// Parse a &str from an option and handle the error
pub(crate) fn parse_str(value: Option<&str>, index: usize) -> Result<&str> {
    let item = value.ok_or_else(|| Error::Parse(format!("No item found at position {}", index)))?;

    Ok(item)
}
