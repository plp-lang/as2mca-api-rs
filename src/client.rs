use std::{sync::Mutex, time::Duration};

use reqwest::{
  Url,
  header::{CONTENT_TYPE, HeaderMap},
};

use crate::{api::session::SessionApi, error::Result};

pub struct Client {
  cl: reqwest::Client,
  pub(crate) base_url: Url,
  pub(crate) session_id: Mutex<Option<String>>,
}

impl Client {
  #[inline]
  #[must_use]
  pub const fn client(&self) -> &reqwest::Client {
    &self.cl
  }

  pub(crate) fn endpoint(&self, path: &str) -> Result<Url> {
    let base = self.base_url.as_str().trim_end_matches('/');
    let path = path.trim_start_matches('/');
    let full_url = format!("{base}/{path}");
    Ok(Url::parse(&full_url)?)
  }

  #[must_use]
  pub fn builder() -> ClientBuilder {
    ClientBuilder::default()
  }

  pub const fn session(&self) -> SessionApi<'_> {
    SessionApi { client: self }
  }
}

pub struct ClientBuilder {
  base_url: String,
  timeout: Duration,
}

impl Default for ClientBuilder {
  fn default() -> Self {
    Self {
      base_url: "https://api.example.com".to_string(),
      timeout: Duration::from_secs(30),
    }
  }
}

impl ClientBuilder {
  #[must_use]
  pub fn base_url(mut self, url: impl Into<String>) -> Self {
    self.base_url = url.into();
    self
  }

  #[must_use]
  pub fn timeout(mut self, timeout: impl Into<Duration>) -> Self {
    self.timeout = timeout.into();
    self
  }

  /// # Errors
  pub fn build(self) -> Result<Client> {
    let base_url = Url::parse(&self.base_url)?;

    let mut headers = HeaderMap::new();
    let content_type = "text/xml; charset=utf-8".parse()?;
    headers.insert(CONTENT_TYPE, content_type);

    let cl = reqwest::Client::builder()
      .timeout(self.timeout)
      .cookie_store(true)
      .user_agent("as2mca-api-rs/0.1.0")
      .default_headers(headers)
      .build()?;

    Ok(Client {
      cl,
      base_url,
      session_id: Mutex::new(None),
    })
  }
}
