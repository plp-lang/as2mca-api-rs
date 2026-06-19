use std::fmt::Write;
use std::time::Duration;

use cookie::Cookie;
use reqwest::{
  Url,
  header::{CONTENT_TYPE, HeaderMap},
};
use tracing::instrument;

use crate::{
  error::{Error, Result},
  models::{
    Credentials, SessionId,
    requests::{
      AuthenticationURLGet, ClassChildrenGet, ClassViewsGet, Disconnect, GuidesGet, GuidesGroupsGet,
      NetworkInformationSet, NovoAllowedCheck, ProtocolInfoGet, Request, RequestKind, SessionInit, SystemCoreInfoGet,
      SystemNetAddressSet, SystemOptionEnabledCheck, SystemServerVersionGet, SystemSettingsGet,
      SystemUserPrivilegedGet, TypesGet, UserBelongsGroupCheck, UserInfoGet, UserMenuGet, UserProfilePropertyGet,
      XML_HEADER,
    },
    responses::{
      AuthenticationURL, CheckResult, ChildClasses, CoreInfo, Done, Guides, GuidesGroups, NovoAllowedCheckResult,
      OptionInfo, ProtocolInfo, Response, ResponseBody, ServerInfo, Session, Settings, Types, User, UserMenu,
      UserPrivileged, UserProfileProperty, Views,
    },
  },
};

#[derive(Clone)]
pub struct Client {
  pub(crate) cl: reqwest::Client,
  pub(crate) base_url: Url,
}

impl Client {
  #[inline]
  #[must_use]
  pub fn builder() -> ClientBuilder {
    ClientBuilder::default()
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "class_children_get"))]
  pub async fn class_children_get(&self, class_children_get: &ClassChildrenGet) -> Result<ChildClasses> {
    self
      .api(&Request {
        body: RequestKind::ClassChildrenGet(class_children_get.clone()),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "class_views_get"))]
  pub async fn class_views_get(&self, class_views_get: &ClassViewsGet) -> Result<Views> {
    self
      .api(&Request {
        body: RequestKind::ClassViewsGet(class_views_get.clone()),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "user_menu_get"))]
  pub async fn user_menu_get(&self, session_id: &SessionId) -> Result<UserMenu> {
    self
      .api(&Request {
        body: RequestKind::UserMenuGet(UserMenuGet {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "guides_get"))]
  pub async fn guides_get(&self, session_id: &SessionId) -> Result<Guides> {
    self
      .api(&Request {
        body: RequestKind::GuidesGet(GuidesGet {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "guides_groups_get"))]
  pub async fn guides_groups_get(&self, session_id: &SessionId) -> Result<GuidesGroups> {
    self
      .api(&Request {
        body: RequestKind::GuidesGroupsGet(GuidesGroupsGet {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "types_get"))]
  pub async fn types_get(&self, session_id: &SessionId) -> Result<Types> {
    self
      .api(&Request {
        body: RequestKind::TypesGet(TypesGet {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "user_belongs_group_check"))]
  pub async fn user_belongs_group_check(
    &self,
    user_belongs_group_check: &UserBelongsGroupCheck,
  ) -> Result<CheckResult> {
    self
      .api(&Request {
        body: RequestKind::UserBelongsGroupCheck(user_belongs_group_check.clone()),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_option_enabled_check"))]
  pub async fn system_option_enabled_check(
    &self,
    system_option_enabled_check: &SystemOptionEnabledCheck,
  ) -> Result<OptionInfo> {
    self
      .api(&Request {
        body: RequestKind::SystemOptionEnabledCheck(system_option_enabled_check.clone()),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "user_profile_property_get"))]
  pub async fn user_profile_property_get(
    &self,
    user_profile_property: &UserProfilePropertyGet,
  ) -> Result<UserProfileProperty> {
    self
      .api(&Request {
        body: RequestKind::UserProfilePropertyGet(user_profile_property.clone()),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "network_information_set"))]
  pub async fn network_information_set(&self, network_info: &NetworkInformationSet) -> Result<Done> {
    self
      .api(&Request {
        body: RequestKind::NetworkInformationSet(network_info.clone()),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_user_privileged_get"))]
  pub async fn system_user_privileged_get(&self, session_id: &SessionId) -> Result<UserPrivileged> {
    self
      .api(&Request {
        body: RequestKind::SystemUserPrivilegedGet(SystemUserPrivilegedGet {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "novo_allowed_check"))]
  pub async fn novo_allowed_check(&self, session_id: &SessionId) -> Result<NovoAllowedCheckResult> {
    self
      .api(&Request {
        body: RequestKind::NovoAllowedCheck(NovoAllowedCheck {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_net_address_set"))]
  pub async fn system_net_address_set(&self, system_net_address: &SystemNetAddressSet) -> Result<Done> {
    self
      .api(&Request {
        body: RequestKind::SystemNetAddressSet(system_net_address.clone()),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "user_info_get"))]
  pub async fn user_info_get(&self, session_id: &SessionId) -> Result<User> {
    self
      .api(&Request {
        body: RequestKind::UserInfoGet(UserInfoGet {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "authentication_url_get"))]
  pub async fn authentication_url_get(&self) -> Result<AuthenticationURL> {
    self
      .api(&Request {
        body: RequestKind::AuthenticationURLGet(AuthenticationURLGet {}),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "session_init"))]
  pub async fn session_init(&self, alive_active_session: Option<bool>) -> Result<Session> {
    self
      .api(&Request {
        body: RequestKind::SessionInit(SessionInit { alive_active_session }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "session_deinit"))]
  pub async fn session_deinit(&self, session_id: &SessionId) -> Result<Done> {
    self
      .api(&Request {
        body: RequestKind::Disconnect(Disconnect {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "protocol_info_get"))]
  pub async fn protocol_info_get(&self, session_id: &SessionId) -> Result<ProtocolInfo> {
    self
      .api(&Request {
        body: RequestKind::ProtocolInfoGet(ProtocolInfoGet {}),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_server_version_get"))]
  pub async fn system_server_version_get(&self, session_id: &SessionId) -> Result<ServerInfo> {
    self
      .api(&Request {
        body: RequestKind::SystemServerVersionGet(SystemServerVersionGet {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_core_info_get"))]
  pub async fn system_core_info_get(&self, session_id: &SessionId) -> Result<CoreInfo> {
    self
      .api(&Request {
        body: RequestKind::SystemCoreInfoGet(SystemCoreInfoGet {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_settings_get"))]
  pub async fn system_settings_get(&self, session_id: &SessionId) -> Result<Settings> {
    self
      .api(&Request {
        body: RequestKind::SystemSettingsGet(SystemSettingsGet {
          session_id: session_id.clone(),
        }),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "authbasic"))]
  pub async fn authbasic(&self, Credentials { username, password }: &Credentials) -> Result<SessionId> {
    let url = self.endpoint("/authbasic")?;
    tracing::debug!(url = url.to_string(), "-> started processing request");

    let response = self
      .cl
      .get(url)
      .basic_auth(username, Some(password))
      .send()
      .await?
      .error_for_status()?;

    let headers = response.headers();

    tracing::debug!(
      headers = headers.iter().fold(String::new(), |mut out, (name, value)| {
        let value_str = value.to_str().unwrap_or("<invalid UTF-8>");
        let _ = writeln!(out, "{name}: {value_str}");
        out
      }),
      "<- finished processing request"
    );

    let session_id = extract_sessionid_from_headers(headers).ok_or(Error::NotFoundSessionId)?;
    Ok(SessionId::new(session_id))
  }

  #[inline]
  pub(crate) fn endpoint(&self, path: &str) -> Result<Url> {
    let base = self.base_url.as_str().trim_end_matches('/');
    let p = path.trim_start_matches('/');
    let full_url = format!("{base}/{p}");
    Ok(Url::parse(&full_url)?)
  }

  #[instrument(skip(self, payload), err)]
  pub(crate) async fn api<T, U>(&self, payload: &T) -> Result<U>
  where
    T: serde::Serialize + Sync,
    U: serde::de::DeserializeOwned + Clone,
  {
    let url = self.endpoint("/api")?;
    let body = format!("{}{}", XML_HEADER, quick_xml::se::to_string(payload)?);
    tracing::debug!(
      url = url.to_string(),
      len = body.len(),
      request = body,
      "-> started processing request"
    );

    let response = self.cl.post(url).body(body).send().await?.error_for_status()?;
    let status_code = response.status();
    let response_text = response.text().await?;

    tracing::debug!(
      len = response_text.len(),
      response = response_text,
      "<- finished processing request"
    );
    let parsed: Response<U> = quick_xml::de::from_str(&response_text)?;
    match &parsed.body {
      ResponseBody::Ok(body) => Ok(body.clone()),
      ResponseBody::Error(body) => Err(Error::Api {
        status_code,
        message: body.text.clone(),
        details: body.body.text.clone(),
      }),
    }
  }
}

pub struct ClientBuilder {
  base_url: String,
  timeout: Duration,
  connect_timeout: Duration,
}

impl Default for ClientBuilder {
  fn default() -> Self {
    Self {
      base_url: "https://api.example.com".to_string(),
      timeout: Duration::from_secs(30),
      connect_timeout: Duration::from_secs(30),
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

  #[must_use]
  pub fn connect_timeout(mut self, connect_timeout: impl Into<Duration>) -> Self {
    self.connect_timeout = connect_timeout.into();
    self
  }

  /// # Errors
  pub fn build(self) -> Result<Client> {
    let base_url = Url::parse(&self.base_url)?;

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);

    let cl = reqwest::Client::builder()
      .connect_timeout(self.connect_timeout)
      .timeout(self.timeout)
      .cookie_store(true)
      .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
      .default_headers(headers)
      .build()?;

    Ok(Client { cl, base_url })
  }
}

#[inline]
#[must_use]
pub(crate) fn extract_sessionid_from_headers(headers: &HeaderMap) -> Option<String> {
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
