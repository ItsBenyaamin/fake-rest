use crate::error::Error;

pub fn get_key_value(content: &str, delimiter: char) -> Result<(String, String), Error> {
    let mut split = content.split(delimiter);
    let key = match split.next() {
        Some(key) => key.trim().to_string(),
        None => return Err(Error::ParsingError(
            format!("cant parse this header: `{}`", content)
        ))
    };

    let value = match split.next() {
        Some(value) => value.trim().to_string(),
        None => return Err(Error::ParsingError(
            format!("cant parse this header: `{}`", content)
        ))
    };
    Ok((key, value))
}

pub fn get_key_optional_value(content: &str, delimiter: char) -> Result<(String, String), Error> {
    let mut split = content.split(delimiter);
    let key = match split.next() {
        Some(key) => key.trim().to_string(),
        None => return Err(Error::ParsingError(
            format!("cant parse this header: `{}`", content)
        ))
    };

    let value = match split.next() {
        Some(value) => value.trim().to_string(),
        None => String::new()
    };
    Ok((key, value))
}