pub mod sanitizer;

use std::str::FromStr;
use url::{ParseError, Url};

pub fn to_url(str: &str) -> Result<Url, ParseError> {
  match !str.starts_with("https://") {
    true => Url::from_str(format!("https://{}", str).as_str()),
    false => Url::from_str(str),
  }
}
