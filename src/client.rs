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
      AuthenticationURLGet, ClassChildrenGet, ClassMethodsGet, ClassMethodsGroupsUserGet, ClassNeedCollectionIDCheck,
      ClassStatesGet, ClassTransitionsGet, ClassViewsGet, DebugTextGet, Disconnect, GuidesGet, GuidesGroupsGet,
      NetworkInformationSet, NovoAllowedCheck, ObjectBackwardReferencesGet, ObjectClassAndArchiveKeyGet, PipeTextGet,
      ProtocolInfoGet, Request, SessionInit, SystemCoreInfoGet, SystemNetAddressSet, SystemOptionEnabledCheck,
      SystemServerVersionGet, SystemSettingGet, SystemSettingsGet, SystemUserPrivilegedGet, TypesGet,
      UserBelongsGroupCheck, UserInfoGet, UserMenuGet, UserProfilePropertyGet, ViewColumnsGet, ViewDataGetCancelable,
      XML_HEADER,
    },
    responses::{
      AuthenticationURL, BackwardReference, BackwardReferences, CheckResult, ChildClasses, Class, Column, Columns,
      CoreInfo, DebugText, Done, GuideClass, Guides, GuidesGroup, GuidesGroups, Method, Methods, MethodsGroups,
      NovoAllowedCheckResult, ObjectClassAndArchiveKey, OptionInfo, PipeText, ProtocolInfo, Response, ResponseBody,
      Row, ServerInfo, Session, Setting, Settings, States, Transitions, Types, User, UserMenu, UserPrivileged,
      UserProfileProperty, View, ViewData, Views,
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
  #[instrument(skip(self), err, fields(method = "object_class_and_archive_key_get"))]
  pub async fn object_class_and_archive_key_get(
    &self,
    req: &ObjectClassAndArchiveKeyGet,
  ) -> Result<ObjectClassAndArchiveKey> {
    self.api::<_, ObjectClassAndArchiveKey>(req).await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "debug_text_get"))]
  pub async fn debug_text_get(&self, req: &DebugTextGet) -> Result<String> {
    self.api::<_, DebugText>(req).await.map(|v| v.value)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_setting_get"))]
  pub async fn system_setting_get(&self, req: &SystemSettingGet) -> Result<Setting> {
    self.api(&req).await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "pipe_text_get"))]
  pub async fn pipe_text_get(&self, req: &PipeTextGet) -> Result<String> {
    self.api::<_, PipeText>(req).await.map(|v| v.value)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "object_backward_references_get"))]
  pub async fn object_backward_references_get(
    &self,
    req: &ObjectBackwardReferencesGet,
  ) -> Result<Vec<BackwardReference>> {
    self.api::<_, BackwardReferences>(&req).await.map(|v| v.body)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "view_data_get_cancelable"))]
  pub async fn view_data_get_cancelable(&self, req: &ViewDataGetCancelable) -> Result<Vec<Row>> {
    self.api::<_, ViewData>(&req).await.map(|v| v.body)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "class_transitions_get"))]
  pub async fn class_transitions_get(&self, req: &ClassTransitionsGet) -> Result<Transitions> {
    self.api::<_, Transitions>(&req).await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "class_states_get"))]
  pub async fn class_states_get(&self, req: &ClassStatesGet) -> Result<States> {
    self.api::<_, States>(&req).await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "view_columns_get"))]
  pub async fn view_columns_get(&self, req: &ViewColumnsGet) -> Result<Vec<Column>> {
    self.api::<_, Columns>(&req).await.map(|v| v.body)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "class_need_collection_id_check"))]
  pub async fn class_need_collection_id_check(&self, req: &ClassNeedCollectionIDCheck) -> Result<String> {
    self.api::<_, CheckResult>(&req).await.map(|v| v.value)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "class_methods_get"))]
  pub async fn class_methods_get(&self, req: &ClassMethodsGet) -> Result<Vec<Method>> {
    self.api::<_, Methods>(&req).await.map(|v| v.body)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "class_methods_groups_user_get"))]
  pub async fn class_methods_groups_user_get(&self, req: &ClassMethodsGroupsUserGet) -> Result<MethodsGroups> {
    self.api::<_, MethodsGroups>(&req).await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "class_children_get"))]
  pub async fn class_children_get(&self, req: &ClassChildrenGet) -> Result<ChildClasses> {
    self.api::<_, ChildClasses>(&req).await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "class_views_get"))]
  pub async fn class_views_get(&self, req: &ClassViewsGet) -> Result<Vec<View>> {
    self.api::<_, Views>(&req).await.map(|v| v.body)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "user_menu_get"))]
  pub async fn user_menu_get(&self, session_id: &SessionId) -> Result<UserMenu> {
    self
      .api::<_, UserMenu>(&UserMenuGet {
        session_id: session_id.clone(),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "guides_get"))]
  pub async fn guides_get(&self, session_id: &SessionId) -> Result<Vec<GuideClass>> {
    self
      .api::<_, Guides>(&GuidesGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.body)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "guides_groups_get"))]
  pub async fn guides_groups_get(&self, session_id: &SessionId) -> Result<Vec<GuidesGroup>> {
    self
      .api::<_, GuidesGroups>(&GuidesGroupsGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.body)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "types_get"))]
  pub async fn types_get(&self, session_id: &SessionId) -> Result<Vec<Class>> {
    self
      .api::<_, Types>(&TypesGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.body)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "user_belongs_group_check"))]
  pub async fn user_belongs_group_check(&self, req: &UserBelongsGroupCheck) -> Result<String> {
    self.api::<_, CheckResult>(&req).await.map(|v| v.value)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_option_enabled_check"))]
  pub async fn system_option_enabled_check(&self, req: &SystemOptionEnabledCheck) -> Result<String> {
    self.api::<_, OptionInfo>(&req).await.map(|v| v.enabled)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "user_profile_property_get"))]
  pub async fn user_profile_property_get(&self, req: &UserProfilePropertyGet) -> Result<String> {
    self.api::<_, UserProfileProperty>(&req).await.map(|v| v.value)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "network_information_set"))]
  pub async fn network_information_set(&self, req: &NetworkInformationSet) -> Result<()> {
    self.api::<_, Done>(&req).await?;
    Ok(())
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_user_privileged_get"))]
  pub async fn system_user_privileged_get(&self, session_id: &SessionId) -> Result<String> {
    self
      .api::<_, UserPrivileged>(&SystemUserPrivilegedGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.is_privileged)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "novo_allowed_check"))]
  pub async fn novo_allowed_check(&self, session_id: &SessionId) -> Result<String> {
    self
      .api::<_, NovoAllowedCheckResult>(&NovoAllowedCheck {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.value)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_net_address_set"))]
  pub async fn system_net_address_set(&self, req: &SystemNetAddressSet) -> Result<()> {
    self.api::<_, Done>(&req).await?;
    Ok(())
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "user_info_get"))]
  pub async fn user_info_get(&self, session_id: &SessionId) -> Result<User> {
    self
      .api::<_, User>(&UserInfoGet {
        session_id: session_id.clone(),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "authentication_url_get"))]
  pub async fn authentication_url_get(&self) -> Result<String> {
    self
      .api::<_, AuthenticationURL>(&AuthenticationURLGet {})
      .await
      .map(|v| v.url)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "session_init"))]
  pub async fn session_init(&self, alive_active_session: Option<bool>) -> Result<Session> {
    self.api(&SessionInit { alive_active_session }).await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "session_deinit"))]
  pub async fn session_deinit(&self, session_id: &SessionId) -> Result<()> {
    self
      .api::<_, Done>(&Disconnect {
        session_id: session_id.clone(),
      })
      .await?;
    Ok(())
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "protocol_info_get"))]
  pub async fn protocol_info_get(&self, session_id: &SessionId) -> Result<String> {
    self
      .api::<_, ProtocolInfo>(&ProtocolInfoGet {})
      .await
      .map(|v| v.version)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_server_version_get"))]
  pub async fn system_server_version_get(&self, session_id: &SessionId) -> Result<String> {
    self
      .api::<_, ServerInfo>(&SystemServerVersionGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.version)
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_core_info_get"))]
  pub async fn system_core_info_get(&self, session_id: &SessionId) -> Result<CoreInfo> {
    self
      .api::<_, CoreInfo>(&SystemCoreInfoGet {
        session_id: session_id.clone(),
      })
      .await
  }

  /// # Errors
  #[instrument(skip(self), err, fields(method = "system_settings_get"))]
  pub async fn system_settings_get(&self, session_id: &SessionId) -> Result<Vec<Setting>> {
    self
      .api::<_, Settings>(&SystemSettingsGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.body)
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

  #[instrument(skip(self, body), err)]
  pub(crate) async fn api<T, U>(&self, body: &T) -> Result<U>
  where
    T: serde::Serialize + Sync,
    U: serde::de::DeserializeOwned + Clone,
  {
    let url = self.endpoint("/api")?;
    let body = format!("{}{}", XML_HEADER, quick_xml::se::to_string(&Request { body })?);
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
