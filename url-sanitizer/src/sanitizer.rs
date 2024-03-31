use url::Url;

pub enum SanitizeError {
  NoDomain,
  UnacceptableDomain,
}

pub trait UrlSanitizer {
  fn sanitize(&self, url: &Url) -> Result<String, SanitizeError>;
  fn get_domains_list(&self) -> Vec<&'static str>;
}

pub struct YouTubeUrlSanitizer {
}
impl YouTubeUrlSanitizer {
    pub fn new() -> Self {
        Self { }
    }
}

impl UrlSanitizer for YouTubeUrlSanitizer {
  fn sanitize(&self, url: &Url) -> Result<String, SanitizeError> {
    let url_domain = match url.domain() {
      Some(domain) => domain,
      None => {
        return Err(SanitizeError::NoDomain);
      }
    };

    let is_domain_acceptable = self
      .get_domains_list()
      .iter()
      .any(|domain| url_domain == *domain);

    match is_domain_acceptable {
      true => {
        let fixed_query: Vec<(_, _)> = url.query_pairs().filter(|(key, _)| key != "si").collect();

        let mut fixed_url = url.clone();
        fixed_url.set_query(None);

        for (key, value) in fixed_query {
          fixed_url
            .query_pairs_mut()
            .append_pair(&key.to_string()[..], &value.to_string()[..]);
        }

        Ok(fixed_url.to_string())
      }
      false => Err(SanitizeError::UnacceptableDomain),
    }
  }

  fn get_domains_list(&self) -> Vec<&'static str> {
    vec!["youtube.com", "youtu.be"]
  }
}
