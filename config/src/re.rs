use ::serde;
use ::serde::Deserialize;
use ::regex;

use std::borrow::Cow;

pub fn deserialize_to_regex<'de, D: serde::de::Deserializer<'de>>(re_str: D) -> Result<regex::Regex, D::Error> {
    let re_str: Cow<'de, str> = Cow::deserialize(re_str)?;

    Ok(regex::Regex::new(&re_str).map_err(serde::de::Error::custom)?)
}
