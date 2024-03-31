use std::{fmt::Error, str::FromStr};
use url::{ParseError, Url};

trait ToUrl {
  fn to_url(&self) -> Result<Url, ParseError>;
}
impl ToUrl for &str {
  fn to_url(&self) -> Result<Url, ParseError> {
    Url::from_str(self)
  }
}

pub fn is_valid_url(str: &str) -> bool {
  match str.to_url() {
    Ok(_) => true,
    Err(_) => false,
  }
}

pub fn sanitize(_url: &Url) -> Result<Url, Error> {
  todo!()
}