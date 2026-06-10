use cookie::Cookie;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::models::requests::{Disconnect, Request, RequestKind, SessionInit};
use crate::models::responses::{Response, ResponseKind};

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

pub struct SessionApi<'a> {
  pub(crate) client: &'a Client,
}

impl SessionApi<'_> {
  #[inline]
  #[must_use]
  pub const fn client(&self) -> &reqwest::Client {
    self.client.client()
  }

  /// # Errors
  pub async fn authbasic(&self, Credentials { username, password }: &Credentials) -> Result<()> {
    let url = self.client.endpoint("/authbasic")?;

    let response = self
      .client()
      .get(url)
      .basic_auth(username, Some(password))
      .send()
      .await?
      .error_for_status()?;

    let extracted_sid = response
      .headers()
      .get_all(reqwest::header::SET_COOKIE)
      .iter()
      .filter_map(|h| h.to_str().ok())
      .find_map(|cookie_str| {
        // Крейт cookie корректно распарсит имя, значение, кавычки и кодировку
        Cookie::parse_encoded(cookie_str)
          .ok()
          .filter(|c| c.name() == "JSESSIONID")
          .map(|c| c.value().to_string())
      });

    if let Some(sid) = extracted_sid
      && let Ok(mut lock) = self.client.session_id.lock()
    {
      *lock = Some(sid);
    }

    Ok(())
  }

  /// # Errors
  pub async fn init(&self, alive_active_session: bool) -> Result<Session> {
    let url = self.client.endpoint("/api")?;
    let body = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>".to_owned()
      + &quick_xml::se::to_string(&Request {
        body: RequestKind::SessionInit(SessionInit {
          alive_active_session: alive_active_session.to_string(),
        }),
      })?;

    let response = self.client().post(url).body(body).send().await?.error_for_status()?;
    let status_code = response.status();
    let response_text = response.text().await?;

    let parsed: Response = quick_xml::de::from_str(&response_text)?;
    match parsed.body {
      ResponseKind::Session(session) => Ok(Session {
        session_id: session.id,
        debug_pipe_name: session.debug_pipe_name,
      }),
      ResponseKind::Error(error) => Err(Error::Api {
        status_code,
        message: error.text,
        details: error.body.text,
      }),
      _ => Err(Error::UnexpectedResponse(response_text)),
    }
  }

  /// # Errors
  /// # Panics
  pub async fn deinit(&self) -> Result<()> {
    let url = self.client.endpoint("/api")?;

    let session_id = self
      .client
      .session_id
      .lock()
      .unwrap()
      .clone()
      .ok_or(Error::NotAuthenticated)?;

    let body = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>".to_owned()
      + &quick_xml::se::to_string(&Request {
        body: RequestKind::Disconnect(Disconnect { session_id }),
      })?;

    let response = self.client().post(url).body(body).send().await?.error_for_status()?;
    let status_code = response.status();
    let response_text = response.text().await?;

    let parsed: Response = quick_xml::de::from_str(&response_text)?;
    match parsed.body {
      ResponseKind::Done(_) => Ok(()),
      ResponseKind::Error(error) => Err(Error::Api {
        status_code,
        message: error.text,
        details: error.body.text,
      }),
      _ => Err(Error::UnexpectedResponse(response_text)),
    }
  }
}
