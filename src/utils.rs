use std::{fmt::Debug, str::FromStr};

use crate::error::{Error, Result};

pub(crate) fn parse_number<'a, T>(value: Option<&'a str>, index: usize) -> Result<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let item = parse_str(value, index)?;
    let result = item
        .parse::<T>()
        .map_err(|e| Error::Parse(format!("Error parsing '{}': {:?}", item, e)))?;

    Ok(result)
}

pub(crate) fn parse_str<'a>(value: Option<&'a str>, index: usize) -> Result<&str> {
    let item = value.ok_or_else(|| Error::Parse(format!("No item found at position {}", index)))?;

    Ok(item)
}
