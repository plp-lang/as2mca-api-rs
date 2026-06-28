use std::fmt::Write;
use std::time::Duration;

use reqwest::{
  Url,
  header::{CONTENT_TYPE, HeaderMap},
};
use tracing::instrument;

use crate::{
  error::{Error, Result},
  models::{
    requests::{
      AuthenticationURLGet, ClassChildrenGet, ClassMethodsGet, ClassMethodsGroupsUserGet, ClassNeedCollectionIDCheck,
      ClassStatesGet, ClassTransitionsGet, ClassViewsGet, Credentials, DebugTextGet, Disconnect, GuidesGet,
      GuidesGroupsGet, NetworkInformationSet, NovoAllowedCheck, ObjectBackwardReferencesGet,
      ObjectClassAndArchiveKeyGet, PipeTextGet, ProtocolInfoGet, Request, SessionId, SessionInit, SystemCoreInfoGet,
      SystemNetAddressSet, SystemOptionEnabledCheck, SystemServerVersionGet, SystemSettingGet, SystemSettingsGet,
      SystemUserPrivilegedGet, TypesGet, UserBelongsGroupCheck, UserInfoGet, UserMenuGet, UserProfilePropertyGet,
      ViewColumnsGet, ViewDataGetCancelable, XML_HEADER,
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
  pub(crate) client: reqwest::Client,
  pub(crate) base_url: Url,
}

impl Client {
  /// Создает новый экземпляр клиента подключения к серверу.
  ///
  /// По умолчанию настраиваются:
  /// - `Content-Type: text/xml; charset=utf-8`
  /// - Включенное хранилище cookie
  /// - Таймауты по 30 секунд
  /// - User-Agent из env `$CARGO_PKG_NAME/$CARGO_PKG_VERSION`
  ///
  /// # Errors
  /// Возвращает ошибку, если `base_url` невалиден или если не удается собрать `reqwest::Client`.
  pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
    let mut base_url = Url::parse(base_url.as_ref()).map_err(|e| Error::UrlParseError(e.to_string()))?;

    if !base_url.path().ends_with('/') {
      let mut path = base_url.path().to_string();
      path.push('/');
      base_url.set_path(&path);
    }

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse()?);

    let client = reqwest::Client::builder()
      .connect_timeout(Duration::from_secs(30))
      .timeout(Duration::from_secs(30))
      .cookie_store(true)
      .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
      .default_headers(headers)
      .build()?;

    Ok(Self { client, base_url })
  }

  /// Создает новый экземпляр клиента с пользовательским `reqwest::Client`.
  ///
  /// *Важно:* При передаче своего клиента, убедитесь, что вы добавили заголовок
  /// `Content-Type: text/xml; charset=utf-8` в `default_headers` и включили хранилище cookie.
  ///
  /// # Examples
  /// ```no_run
  ///  use std::time::Duration;
  ///  use reqwest::header::{CONTENT_TYPE, HeaderMap};
  ///  use as2mca_api::client::Client;
  ///
  ///  let mut headers = HeaderMap::new();
  ///  headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse().unwrap());
  ///
  ///  let client = reqwest::Client::builder()
  ///    .connect_timeout(Duration::from_secs(60))
  ///    .cookie_store(true)
  ///    .default_headers(headers)
  ///    .build()
  ///    .unwrap();
  ///
  ///  let client = Client::with_client("http://localhost:3000/platform2mca", client).unwrap();
  /// ```
  ///
  /// # Errors
  /// Возвращает ошибку, если `base_url` невалиден.
  pub fn with_client(base_url: impl Into<String>, client: reqwest::Client) -> Result<Self> {
    let mut base_url = Url::parse(&base_url.into()).map_err(|e| Error::UrlParseError(e.to_string()))?;

    if !base_url.path().ends_with('/') {
      let mut path = base_url.path().to_string();
      path.push('/');
      base_url.set_path(&path);
    }

    Ok(Self { client, base_url })
  }

  //====================================================================================================================
  // Сессия
  //====================================================================================================================

  /// Авторизация в системе методом Basic Auth.
  ///
  /// Сервер устанавливает в cookies `JSESSIONID`, вне зависимости от валидности авторизационных данных.
  ///
  /// # Errors
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::NotFoundSessionId`], если сервер не установил cookie `JSESSIONID` в ответе.
  #[instrument(skip(self), err, fields(method = "authbasic"))]
  pub async fn authbasic(&self, Credentials { username, password }: &Credentials) -> Result<SessionId> {
    let url = self.endpoint("/authbasic")?;

    tracing::debug!(url = url.to_string(), "-> started processing request");

    let response = self
      .client
      .get(url)
      .basic_auth(username, Some(password))
      .send()
      .await?
      .error_for_status()?;

    let headers = response.headers();
    let session_id = headers
      .get_all(reqwest::header::SET_COOKIE)
      .iter()
      .filter_map(|h| h.to_str().ok())
      .find_map(|cookie_str| {
        cookie_str.split(';').find_map(|part| {
          let mut kv = part.splitn(2, '=');
          let key = kv.next()?.trim();
          if key.eq_ignore_ascii_case("JSESSIONID") {
            Some(kv.next()?.trim().to_string())
          } else {
            None
          }
        })
      })
      .ok_or(Error::NotFoundSessionId)?;

    tracing::debug!(
      session_id = session_id,
      headers = headers.iter().fold(String::new(), |mut out, (name, value)| {
        let value_str = value.to_str().unwrap_or("<invalid UTF-8>");
        let _ = writeln!(out, "{name}: {value_str}");
        out
      }),
      "<- finished processing request"
    );

    Ok(SessionId::new(session_id))
  }

  /// Активация сессии.
  ///
  /// Сервер валидирует авторизационные данные переданные в [`Client::authbasic`] и возвращает
  /// наименование отладочного канала [`DebugPipeName`].
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Узнать что делает `alive_active_session`
  #[instrument(skip(self), err, fields(method = "session_init"))]
  pub async fn session_init(&self, alive_active_session: Option<bool>) -> Result<Session> {
    self.api(&SessionInit { alive_active_session }).await
  }

  /// Деативирует сессию, делая её невалидной для последующих запросов.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "session_deinit"))]
  pub async fn session_deinit(&self, session_id: &SessionId) -> Result<()> {
    self
      .api::<_, Done>(&Disconnect {
        session_id: session_id.clone(),
      })
      .await?;
    Ok(())
  }

  /// Устанавливает для сессии пользователя следующую информацию:
  /// - `client_name` - hostname устройства пользователя;
  /// - `client_ip` - локальный ip-адрес устройства пользователя;
  /// - `client_user` - username пользователя, например `echo $USER` в Linux;
  /// - `module_name` - наименование клиентского приложения, например `ЦФТ - Навигатор 6.0.121.84`.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Узнать разницу между ip_address из SystemNetAddressSet
  #[instrument(skip(self), err, fields(method = "network_information_set"))]
  pub async fn network_information_set(&self, req: &NetworkInformationSet) -> Result<()> {
    self.api::<_, Done>(&req).await?;
    Ok(())
  }

  /// Устанавливает для сессии пользователя следующую информацию:
  /// - `mac_address` - mac-адрес устройства пользователя;
  /// - `ip_address` - локальный ip-адрес устройства пользователя.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Узнать разницу между ip_address из NetworkInformationSet
  #[instrument(skip(self), err, fields(method = "system_net_address_set"))]
  pub async fn system_net_address_set(&self, req: &SystemNetAddressSet) -> Result<()> {
    self.api::<_, Done>(&req).await?;
    Ok(())
  }

  //====================================================================================================================
  // Получение информации о системе
  //====================================================================================================================

  /// Возвращает версию протокола обмена запросами с API сервера приложений
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  ///
  /// # Notes
  /// Библиотека протестирована с версией протокола `9.54`.
  /// Открытая спецификация различий между версиями протокола отсутствует.
  /// Если вам известна специфика работы с другими версиями, пожалуйста, поделитесь информацией в Issue или Pull Request.
  #[instrument(skip(self), err, fields(method = "protocol_info_get"))]
  pub async fn protocol_info_get(&self, session_id: &SessionId) -> Result<String> {
    self
      .api::<_, ProtocolInfo>(&ProtocolInfoGet {})
      .await
      .map(|v| v.version)
  }

  /// Возвращает версию сервера приложений.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "system_server_version_get"))]
  pub async fn system_server_version_get(&self, session_id: &SessionId) -> Result<String> {
    self
      .api::<_, ServerInfo>(&SystemServerVersionGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.version)
  }

  /// Возвращает информацию о ядре системы (ТЯ).
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Детализировать описание возвращаемых полей и их назначение.
  #[instrument(skip(self), err, fields(method = "system_core_info_get"))]
  pub async fn system_core_info_get(&self, session_id: &SessionId) -> Result<CoreInfo> {
    self
      .api::<_, CoreInfo>(&SystemCoreInfoGet {
        session_id: session_id.clone(),
      })
      .await
  }

  /// Возвращает весь настройки системы в формате ключ-значение.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Объяснить, откуда берутся настройки и как они формируются на стороне сервера.
  #[instrument(skip(self), err, fields(method = "system_settings_get"))]
  pub async fn system_settings_get(&self, session_id: &SessionId) -> Result<Vec<Setting>> {
    self
      .api::<_, Settings>(&SystemSettingsGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.body)
  }

  /// Возвращает значение конкретной настройки системы по её ключу.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Объяснить, откуда берутся настройки и как они формируются на стороне сервера.
  #[instrument(skip(self), err, fields(method = "system_setting_get"))]
  pub async fn system_setting_get(&self, req: &SystemSettingGet) -> Result<Option<String>> {
    self.api::<_, Setting>(&req).await.map(|s| s.value)
  }

  /// Возвращает относительный URL до эндпоинта авторизации.
  ///
  /// Например, ответ может выглядеть следующим образом: `/platform2mca/authbasic`.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "authentication_url_get"))]
  pub async fn authentication_url_get(&self) -> Result<String> {
    self
      .api::<_, AuthenticationURL>(&AuthenticationURLGet {})
      .await
      .map(|v| v.url)
  }

  /// Проверяет доступность функционала NOVO для текущей сессии.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Уточнить, что именно означает возвращаемое значение?
  #[instrument(skip(self), err, fields(method = "novo_allowed_check"))]
  pub async fn novo_allowed_check(&self, session_id: &SessionId) -> Result<String> {
    self
      .api::<_, NovoAllowedCheckResult>(&NovoAllowedCheck {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.value)
  }

  /// Проверяет, включена ли указанная системная опция.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Объяснить, откуда берутся опции и как они конфигурируются на сервере
  #[instrument(skip(self), err, fields(method = "system_option_enabled_check"))]
  pub async fn system_option_enabled_check(&self, req: &SystemOptionEnabledCheck) -> Result<bool> {
    self.api::<_, OptionInfo>(&req).await.map(|v| v.enabled)
  }

  //====================================================================================================================
  // Получение информации о пользователе
  //====================================================================================================================

  /// Узнать является ли пользователь привелигированным.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "system_user_privileged_get"))]
  pub async fn system_user_privileged_get(&self, session_id: &SessionId) -> Result<bool> {
    self
      .api::<_, UserPrivileged>(&SystemUserPrivilegedGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.is_privileged)
  }

  /// Возвращает следующую информацию о пользователе:
  /// - `name` - полное имя пользователя, например ФИО;
  /// - `short_name` - короткое имя пользователя в системе;
  /// - `properties` - параметры пользователя, перечесление строк с разделителем `|`, например `|ADMIN|CONTEXT|PICKER|PROFILE DEFAULT|SESSION|`
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Объяснить, откуда берутся свойства пользователя, их значение и как они формируются на стороне сервера.
  #[instrument(skip(self), err, fields(method = "user_info_get"))]
  pub async fn user_info_get(&self, session_id: &SessionId) -> Result<User> {
    self
      .api::<_, User>(&UserInfoGet {
        session_id: session_id.clone(),
      })
      .await
  }

  /// Возвращает значение конкретной настройки пользователя по её ключу.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Объяснить, откуда берутся свойства пользователя, их значение и как они формируются на стороне сервера.
  #[instrument(skip(self), err, fields(method = "user_profile_property_get"))]
  pub async fn user_profile_property_get(&self, req: &UserProfilePropertyGet) -> Result<String> {
    self.api::<_, UserProfileProperty>(&req).await.map(|v| v.value)
  }

  /// Проверяет, входит ли пользователь в указанную группу.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "user_belongs_group_check"))]
  pub async fn user_belongs_group_check(&self, req: &UserBelongsGroupCheck) -> Result<String> {
    self.api::<_, CheckResult>(&req).await.map(|v| v.value)
  }

  //====================================================================================================================
  // Отладка
  //====================================================================================================================

  /// Получить текст сгенерированный сервером по наименованию канала.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "pipe_text_get"))]
  pub async fn pipe_text_get(&self, req: &PipeTextGet) -> Result<String> {
    self.api::<_, PipeText>(req).await.map(|v| v.value)
  }

  /// Получить отладочную информацию.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  // TODO: Найти где и как эта информация используется и генерируется.
  #[instrument(skip(self), err, fields(method = "debug_text_get"))]
  pub async fn debug_text_get(&self, req: &DebugTextGet) -> Result<String> {
    self.api::<_, DebugText>(req).await.map(|v| v.value)
  }

  //====================================================================================================================
  // Метаданные классов и объектов
  //====================================================================================================================

  /// Возвращает короткое имя базового ТБП и ключ архива для указанного экземпляра.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "object_class_and_archive_key_get"))]
  pub async fn object_class_and_archive_key_get(
    &self,
    req: &ObjectClassAndArchiveKeyGet,
  ) -> Result<ObjectClassAndArchiveKey> {
    self.api::<_, ObjectClassAndArchiveKey>(req).await
  }

  /// Возвращает список обратных ссылок на указанный экземпляр (которые ссылаются на экземпляр).
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "object_backward_references_get"))]
  pub async fn object_backward_references_get(
    &self,
    req: &ObjectBackwardReferencesGet,
  ) -> Result<Vec<BackwardReference>> {
    self.api::<_, BackwardReferences>(&req).await.map(|v| v.body)
  }

  /// Возвращает информацию о возможных переходах между состояниями для указанного ТБП.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "class_transitions_get"))]
  pub async fn class_transitions_get(&self, req: &ClassTransitionsGet) -> Result<Transitions> {
    self.api::<_, Transitions>(&req).await
  }

  /// Возвращает список состояний для указанного ТБП.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "class_states_get"))]
  pub async fn class_states_get(&self, req: &ClassStatesGet) -> Result<States> {
    self.api::<_, States>(&req).await
  }

  /// Проверяет, требуется ли указывать `collectionid` для ТБП.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "class_need_collection_id_check"))]
  pub async fn class_need_collection_id_check(&self, req: &ClassNeedCollectionIDCheck) -> Result<String> {
    self.api::<_, CheckResult>(&req).await.map(|v| v.value)
  }

  /// Возвращает список операций, доступных для указанного ТБП.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "class_methods_get"))]
  pub async fn class_methods_get(&self, req: &ClassMethodsGet) -> Result<Vec<Method>> {
    self.api::<_, Methods>(&req).await.map(|v| v.body)
  }

  /// Возвращает группы операций пользователя для указанного ТБП.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "class_methods_groups_user_get"))]
  pub async fn class_methods_groups_user_get(&self, req: &ClassMethodsGroupsUserGet) -> Result<MethodsGroups> {
    self.api::<_, MethodsGroups>(&req).await
  }

  /// Возвращает список дочерних ТБП для указанного ТБП.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "class_children_get"))]
  pub async fn class_children_get(&self, req: &ClassChildrenGet) -> Result<ChildClasses> {
    self.api::<_, ChildClasses>(&req).await
  }

  //====================================================================================================================
  // Представления и данные
  //====================================================================================================================

  /// Получает данные представления.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "view_data_get_cancelable"))]
  pub async fn view_data_get_cancelable(&self, req: &ViewDataGetCancelable) -> Result<Vec<Row>> {
    self.api::<_, ViewData>(&req).await.map(|v| v.body)
  }

  /// Возвращает список колонок для указанного представления.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "view_columns_get"))]
  pub async fn view_columns_get(&self, req: &ViewColumnsGet) -> Result<Vec<Column>> {
    self.api::<_, Columns>(&req).await.map(|v| v.body)
  }

  /// Возвращает список представлений, доступных для указанного ТБП.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "class_views_get"))]
  pub async fn class_views_get(&self, req: &ClassViewsGet) -> Result<Vec<View>> {
    self.api::<_, Views>(&req).await.map(|v| v.body)
  }

  //====================================================================================================================
  // Навигация, справочники и меню
  //====================================================================================================================

  /// Возвращает структуру пользовательского меню.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "user_menu_get"))]
  pub async fn user_menu_get(&self, session_id: &SessionId) -> Result<UserMenu> {
    self
      .api::<_, UserMenu>(&UserMenuGet {
        session_id: session_id.clone(),
      })
      .await
  }

  /// Возвращает список справочников, доступных пользователю.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "guides_get"))]
  pub async fn guides_get(&self, session_id: &SessionId) -> Result<Vec<GuideClass>> {
    self
      .api::<_, Guides>(&GuidesGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.body)
  }

  /// Возвращает список групп справочников.
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "guides_groups_get"))]
  pub async fn guides_groups_get(&self, session_id: &SessionId) -> Result<Vec<GuidesGroup>> {
    self
      .api::<_, GuidesGroups>(&GuidesGroupsGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.body)
  }

  /// Возвращает список всех доступных ТБП (не справочников).
  ///
  /// # Errors
  /// - [`Error::Api`], если сервер вернул ошибку следующего вида: `<Response><Error Text="..."><ServerErrorInfo Text="..."></Error></Response>`;
  /// - [`Error::Http`], если сеть недоступна, истёк таймаут или сервер вернул статус `4xx/5xx`;
  /// - [`Error::UrlParseError`], если не удалось собрать URL;
  /// - [`Error::XmlSerializeError`], если не удалось собрать тело запроса;
  /// - [`Error::XmlDeserializeError`], если не удалось разобрать тело ответа.
  #[instrument(skip(self), err, fields(method = "types_get"))]
  pub async fn types_get(&self, session_id: &SessionId) -> Result<Vec<Class>> {
    self
      .api::<_, Types>(&TypesGet {
        session_id: session_id.clone(),
      })
      .await
      .map(|v| v.body)
  }

  #[inline]
  pub(crate) fn endpoint(&self, path: &str) -> Result<Url> {
    self
      .base_url
      .join(path.trim_start_matches('/'))
      .map_err(|e| Error::UrlParseError(e.to_string()))
  }

  #[instrument(skip(self, body), err)]
  pub(crate) async fn api<T, U>(&self, body: &T) -> Result<U>
  where
    T: serde::Serialize + Sync,
    U: serde::de::DeserializeOwned + Clone,
  {
    let url = self.endpoint("/api")?;

    let xml_body = quick_xml::se::to_string(&Request { body })?;

    let mut body_bytes = Vec::with_capacity(XML_HEADER.len() + xml_body.len());
    body_bytes.extend_from_slice(XML_HEADER.as_bytes());
    body_bytes.extend_from_slice(xml_body.as_bytes());

    tracing::debug!(
      url = url.to_string(),
      len = body_bytes.len(),
      request = %String::from_utf8_lossy(&body_bytes),
      "-> started processing request"
    );

    let response = self
      .client
      .post(url)
      .body(body_bytes)
      .send()
      .await?
      .error_for_status()?;
    let status_code = response.status();
    let response_bytes = response.bytes().await?;

    tracing::debug!(
      len = response_bytes.len(),
      response = %String::from_utf8_lossy(&response_bytes),
      "<- finished processing request"
    );

    let parsed: Response<U> = quick_xml::de::from_reader(response_bytes.as_ref())?;
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
