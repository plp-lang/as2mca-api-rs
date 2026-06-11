use std::{sync::Mutex, time::Duration};

use cookie::Cookie;
use reqwest::{
  Url,
  header::{CONTENT_TYPE, HeaderMap},
};

use crate::{
  error::{Error, Result},
  models::{
    requests::{Disconnect, GuidesGroupsGet, Request, RequestKind, SessionInit, TypesGet},
    responses::{Class, CoreInfo, GuidesGroup, Response, ResponseKind, ServerInfo, Setting},
  },
};

#[derive(Debug)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

#[derive(Debug)]
pub struct Session {
  pub session_id: String,
  pub debug_pipe_name: String,
}

pub struct Client {
  pub(crate) cl: reqwest::Client,
  pub(crate) base_url: Url,
  pub(crate) jsession_id: Mutex<Option<String>>,
}

impl Client {
  #[inline]
  #[must_use]
  pub fn builder() -> ClientBuilder {
    ClientBuilder::default()
  }

  /// # Panics
  #[inline]
  #[must_use]
  pub fn jsession_id(&self) -> Option<String> {
    self.jsession_id.lock().unwrap().clone()
  }

  #[inline]
  pub(crate) fn set_jsession_id(&self, new_id: String) {
    *self.jsession_id.lock().unwrap() = Some(new_id);
  }

  #[inline]
  pub(crate) fn endpoint(&self, path: &str) -> Result<Url> {
    let base = self.base_url.as_str().trim_end_matches('/');
    let path = path.trim_start_matches('/');
    let full_url = format!("{base}/{path}");
    Ok(Url::parse(&full_url)?)
  }

  /// # Errors
  pub async fn authbasic(&self, Credentials { username, password }: &Credentials) -> Result<()> {
    let url = self.endpoint("/authbasic")?;

    let response = self
      .cl
      .get(url)
      .basic_auth(username, Some(password))
      .send()
      .await?
      .error_for_status()?;

    if let Some(id) = extract_jsession_from_headers(response.headers()) {
      self.set_jsession_id(id);
    }

    Ok(())
  }

  pub(crate) async fn api<T: serde::Serialize + Sync>(&self, payload: &T) -> Result<Response> {
    let url = self.endpoint("/api")?;
    let body = format!(
      "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>{}",
      quick_xml::se::to_string(payload)?
    );

    let response = self.cl.post(url).body(body).send().await?.error_for_status()?;
    let status_code = response.status();
    let response_text = response.text().await?;

    let parsed: Response = quick_xml::de::from_str(&response_text)?;
    match &parsed.body {
      ResponseKind::Error(error) => Err(Error::Api {
        status_code,
        message: error.text.clone(),
        details: error.body.text.clone(),
      }),
      _ => Ok(parsed),
    }
  }

  /// # Errors
  pub async fn session_init(&self, alive_active_session: bool) -> Result<Session> {
    let payload = Request {
      body: RequestKind::SessionInit(SessionInit {
        alive_active_session: alive_active_session.to_string(),
      }),
    };

    let response = self.api(&payload).await?;

    match response.body {
      ResponseKind::Session(session) => Ok(Session {
        session_id: session.id,
        debug_pipe_name: session.debug_pipe_name,
      }),
      _ => Err(Error::UnexpectedResponse("Expected Session Init response".into())),
    }
  }

  /// # Errors
  pub async fn session_deinit(&self) -> Result<()> {
    let session_id = self.jsession_id().ok_or(Error::NotAuthenticated)?;
    let payload = Request {
      body: RequestKind::Disconnect(Disconnect { session_id }),
    };

    let response = self.api(&payload).await?;
    match response.body {
      ResponseKind::Done(_) => Ok(()),
      _ => Err(Error::UnexpectedResponse("Expected Session Deinit response".into())),
    }
  }

  /// # Errors
  pub async fn system_server_version_get(&self) -> Result<ServerInfo> {
    let session_id = self.jsession_id().ok_or(Error::NotAuthenticated)?;
    let payload = Request {
      body: RequestKind::SystemServerVersionGet { session_id },
    };

    let response = self.api(&payload).await?;
    match response.body {
      ResponseKind::ServerInfo(server_info) => Ok(server_info),
      _ => Err(Error::UnexpectedResponse(
        "Expected system server version get response".into(),
      )),
    }
  }

  /// # Errors
  pub async fn system_core_info_get(&self) -> Result<CoreInfo> {
    let session_id = self.jsession_id().ok_or(Error::NotAuthenticated)?;
    let payload = Request {
      body: RequestKind::SystemCoreInfoGet { session_id },
    };

    let response = self.api(&payload).await?;
    match response.body {
      ResponseKind::CoreInfo(core_info) => Ok(core_info),
      _ => Err(Error::UnexpectedResponse(
        "Expected system core info get response".into(),
      )),
    }
  }

  /// # Errors
  pub async fn system_settings_get(&self) -> Result<Vec<Setting>> {
    let session_id = self.jsession_id().ok_or(Error::NotAuthenticated)?;
    let payload = Request {
      body: RequestKind::SystemSettingsGet { session_id },
    };

    let response = self.api(&payload).await?;
    match response.body {
      ResponseKind::Settings { body } => Ok(body),
      _ => Err(Error::UnexpectedResponse(
        "Expected system settings get response".into(),
      )),
    }
  }

  /// # Errors
  pub async fn types_get(&self) -> Result<Vec<Class>> {
    let session_id = self.jsession_id().ok_or(Error::NotAuthenticated)?;
    let payload = Request {
      body: RequestKind::TypesGet(TypesGet { session_id }),
    };

    let response = self.api(&payload).await?;
    match response.body {
      ResponseKind::Types(types) => Ok(types.body),
      _ => Err(Error::UnexpectedResponse("Expected types get response".into())),
    }
  }

  /// # Errors
  pub async fn guides_groups_get(&self) -> Result<Vec<GuidesGroup>> {
    let session_id = self.jsession_id().ok_or(Error::NotAuthenticated)?;
    let payload = Request {
      body: RequestKind::GuidesGroupsGet(GuidesGroupsGet { session_id }),
    };

    let response = self.api(&payload).await?;
    match response.body {
      ResponseKind::GuidesGroups { body } => Ok(body),
      _ => Err(Error::UnexpectedResponse("Expected guides groups get response".into())),
    }
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
      jsession_id: Mutex::new(None),
    })
  }
}

#[inline]
#[must_use]
pub(crate) fn extract_jsession_from_headers(headers: &HeaderMap) -> Option<String> {
  headers
    .get_all(reqwest::header::SET_COOKIE)
    .iter()
    .filter_map(|h| h.to_str().ok())
    .find_map(|cookie_str| {
      Cookie::parse_encoded(cookie_str)
        .ok()
        .filter(|c| c.name() == "JSESSIONID")
        .map(|c| c.value().to_string())
    })
}
