use std::fmt::Write;
use std::time::Duration;

use reqwest::{
  Url,
  header::{CONTENT_TYPE, HeaderMap},
};
use tracing::instrument;

use crate::{
  error::{Error, Result},
  requests::{
    AuthenticationURLGet, ClassChildrenGet, ClassGet, ClassInfo, ClassMethodsGet, ClassMethodsGroupsUserGet,
    ClassNeedCollectionIDCheck, ClassStatesGet, ClassTransitionsGet, ClassViewsGet, ClassesGet, DebugTextGet,
    Disconnect, GuidesGet, GuidesGroupsGet, MethodBegin, MethodClientScriptGet, MethodControlsGet, MethodEnd,
    MethodExecute, MethodParametersGet, MethodValidate, MethodValidateDefault, MethodVariablesGet,
    NetworkInformationSet, NovoAllowedCheck, Object, ObjectBackwardReferencesGet, ObjectClassAndArchiveKeyGet,
    ObjectsLock, ObjectsUnlock, PipeTextGet, ProtocolInfoGet, Request, SessionInit, SystemCoreInfoGet,
    SystemNetAddressSet, SystemOptionEnabledCheck, SystemServerVersionGet, SystemSettingGet, SystemSettingsGet,
    SystemUserPrivilegedGet, TypesGet, UserBelongsGroupCheck, UserInfoGet, UserMenuGet, UserProfilePropertyGet,
    ViewColumnsGet, ViewDataGetCancelable, XML_HEADER,
  },
  responses::{
    BackwardReference, ChildClass, Class, Column, Control, CoreInfo, GuidesGroup, Method, MethodFrame, MethodParameter,
    MethodResult, MethodVariable, MethodsGroup, ObjectClassAndArchiveKey, Response, ResponseBody, Row, Session,
    Setting, State, Transition, User, UserContent, UserMenuItem, Validate, View,
  },
};

/// Клиент для взаимодействия с API сервера приложений.
///
/// Содержит HTTP‑клиент и базовый URL. Все методы выполняют POST‑запросы на эндпоинт `/api`
/// с XML‑телом, соответствующим структурам из модуля `requests`.
///
/// # Важно
/// - Для работы требуется **активная сессия**, полученная через [`Client::authbasic`] и [`Client::session_init`].
#[derive(Clone)]
pub struct Client {
  pub(crate) client: reqwest::Client,
  pub(crate) base_url: Url,
}

impl Client {
  /// Создаёт новый экземпляр клиента с настройками по умолчанию.
  ///
  /// # Настройки по умолчанию
  /// - `Content-Type: text/xml; charset=utf-8`
  /// - Включено хранилище cookie (поддержка сессий)
  /// - Таймауты: `connect_timeout` и `timeout` – 5 секунд
  /// - `User-Agent`: `$CARGO_PKG_NAME/$CARGO_PKG_VERSION`
  ///
  /// # Arguments
  /// * `base_url` – базовый URL сервера (например, `"http://localhost:3000/platform2mca"`).
  ///   Слеш в конце добавляется автоматически, если его нет.
  ///
  /// # Errors
  /// Возвращает [`Error::UrlParseError`], если переданный URL невалиден,
  /// или [`Error::InvalidHeaderValue`], если не удалось установить заголовок.
  ///
  /// # Examples
  /// ```ignore
  /// use as2mca_api::client::Client;
  /// let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// ```
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
      .connect_timeout(Duration::from_secs(5))
      .timeout(Duration::from_secs(5))
      .cookie_store(true)
      .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
      .default_headers(headers)
      .build()?;

    Ok(Self { client, base_url })
  }

  /// Создаёт клиент с пользовательским экземпляром `reqwest::Client`.
  ///
  /// # Важно
  /// При передаче своего клиента необходимо самостоятельно обеспечить:
  /// - Заголовок `Content-Type: text/xml; charset=utf-8`
  /// - Включённое хранилище cookie (для поддержки сессий)
  ///
  /// # Arguments
  /// * `base_url` – базовый URL сервера.
  /// * `client` – предварительно настроенный `reqwest::Client`.
  ///
  /// # Errors
  /// Возвращает [`Error::UrlParseError`], если `base_url` невалиден.
  ///
  /// # Examples
  /// ```ignore
  /// use std::time::Duration;
  /// use reqwest::header::{CONTENT_TYPE, HeaderMap};
  /// use as2mca_api::client::Client;
  ///
  /// let mut headers = HeaderMap::new();
  /// headers.insert(CONTENT_TYPE, "text/xml; charset=utf-8".parse().unwrap());
  ///
  /// let client = reqwest::Client::builder()
  ///   .connect_timeout(Duration::from_secs(60))
  ///   .cookie_store(true)
  ///   .default_headers(headers)
  ///   .build()
  ///   .unwrap();
  ///
  /// let client = Client::with_client("http://localhost:3000/platform2mca", client).unwrap();
  /// ```
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

  /// Выполняет HTTP Basic‑авторизацию на сервере.
  ///
  /// Сервер в любом случае устанавливает cookie `JSESSIONID` (даже при неверных учётных данных),
  /// поэтому этот метод не возвращает ошибку при неудачной аутентификации.
  /// Для проверки успешности следует вызвать [`session_init`](Client::session_init).
  ///
  /// # Arguments
  /// * `username` – имя пользователя
  /// * `password` – пароль
  ///
  /// # Errors
  /// Возвращает [`Error::Http`] при сетевых проблемах или если сервер вернул HTTP‑код 4xx/5xx.
  ///
  /// # Notes
  /// После вызова этого метода сервер установит `JSESSIONID` в cookie клиента.
  /// Для активации сессии необходимо вызвать [`session_init`](Client::session_init).
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// client.authbasic("admin", "password").await.unwrap();
  /// ```
  #[instrument(skip(self), err, fields(method = "authbasic"))]
  pub async fn authbasic(&self, username: &str, password: &str) -> Result<()> {
    let url = self.endpoint("/authbasic")?;

    tracing::trace!(url = url.to_string(), "-> started processing request");

    let response = self
      .client
      .get(url)
      .basic_auth(username, Some(password))
      .send()
      .await?
      .error_for_status()?;

    let headers = response.headers();
    headers
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

    tracing::trace!(
      headers = headers.iter().fold(String::new(), |mut out, (name, value)| {
        let value_str = value.to_str().unwrap_or("<invalid UTF-8>");
        let _ = writeln!(out, "{name}: {value_str}");
        out
      }),
      "<- finished processing request"
    );

    Ok(())
  }

  /// Активирует сессию, проверяя валидность учётных данных.
  ///
  /// После успешного вызова возвращается структура [`Session`], содержащая `session_id`
  /// и имя отладочного канала (`debug_pipe_name`).
  ///
  /// # Errors
  /// Возвращает [`Error::Api`], если сервер вернул ошибку (неверные логин/пароль, блокировка и т.п.).
  /// Также возможны сетевые ошибки и ошибки сериализации/десериализации.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// let session = client.session_init(None).await.unwrap();
  /// println!("Session ID: {}", session.session_id);
  /// ```
  #[instrument(skip(self), err, fields(method = "session_init"))]
  pub async fn session_init(&self, alive_active_session: Option<bool>) -> Result<Session> {
    let body = self.api(&SessionInit { alive_active_session }).await?;
    match body {
      ResponseBody::Session(session) => Ok(session),
      _ => Err(Error::UnexpectedResponse {
        expected: "Session".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Деактивирует сессию, делая её недействительной.
  ///
  /// После вызова все последующие запросы с этим `session_id` будут отклонены.
  /// Cookie `JSESSIONID` на стороне клиента не удаляется – следует очистить хранилище cookie отдельно,
  /// если это необходимо.
  ///
  /// # Arguments
  /// * `session_id` – идентификатор сессии, полученный из [`Session::session_id`].
  ///
  /// # Errors
  /// Возвращает [`Error::Api`], если сессия уже неактивна или невалидна.
  /// Также возможны стандартные сетевые ошибки.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// client.session_deinit(&session.session_id).await.unwrap();
  /// ```
  #[instrument(skip(self), err, fields(method = "session_deinit"))]
  pub async fn session_deinit(&self, session_id: &str) -> Result<()> {
    let body = self.api(&Disconnect { session_id }).await?;
    match body {
      ResponseBody::Done(_) => Ok(()),
      _ => Err(Error::UnexpectedResponse {
        expected: "Done".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Устанавливает сетевую информацию для текущей сессии.
  ///
  /// Эта информация используется сервером для аудита и логирования.
  /// На серверной стороне (Oracle) выполняется примерно такой PL/SQL:
  /// ```pl/sql
  /// BEGIN IBS.nav.network_register_node(:1,:2,:3,:4,:5); commit; END;
  /// ```
  ///
  /// # Arguments
  /// * `req` – структура [`NetworkInformationSet`], содержащая:
  ///   - `client_name` – hostname устройства пользователя
  ///   - `client_ip` – локальный IP‑адрес
  ///   - `client_user` – имя пользователя ОС (например, из `whoami`)
  ///   - `module_name` – название клиентского приложения (например, "ЦФТ - Навигатор 6.0")
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// use as2mca_api::requests::NetworkInformationSet;
  ///
  /// client.network_information_set(&NetworkInformationSet {
  ///   session_id: &session.session_id,
  ///   client_name: "my-host",
  ///   client_ip: "192.168.1.100",
  ///   client_user: "john",
  ///   module_name: "MyApp/1.0",
  /// }).await.unwrap();
  /// ```
  #[instrument(skip(self), err, fields(method = "network_information_set"))]
  pub async fn network_information_set(&self, req: &NetworkInformationSet<'_>) -> Result<()> {
    let body = self.api(req).await?;
    match body {
      ResponseBody::Done(_) => Ok(()),
      _ => Err(Error::UnexpectedResponse {
        expected: "Done".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Устанавливает MAC и IP‑адрес клиента для текущей сессии.
  ///
  /// На серверной стороне (Oracle) выполняется примерно такой PL/SQL:
  /// ```pl/sql
  /// BEGIN DECLARE v varchar2(1); BEGIN v:=IBS.Nav.SetNetAddresses(:1, :2); END; END
  /// ```
  ///
  /// # Arguments
  /// * `req` – структура [`SystemNetAddressSet`], содержащая:
  ///   - `mac_address` – MAC‑адрес устройства (например, "aa:bb:cc:dd:ee:ff")
  ///   - `ip_address` – локальный IP‑адрес
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// use as2mca_api::requests::SystemNetAddressSet;
  ///
  /// client.system_net_address_set(&SystemNetAddressSet {
  ///   session_id: &session.session_id,
  ///   mac_address: "00:11:22:33:44:55",
  ///   ip_address: "192.168.1.100",
  /// }).await.unwrap();
  /// ```
  #[instrument(skip(self), err, fields(method = "system_net_address_set"))]
  pub async fn system_net_address_set(&self, req: &SystemNetAddressSet<'_>) -> Result<()> {
    let body = self.api(req).await?;
    match body {
      ResponseBody::Done(_) => Ok(()),
      _ => Err(Error::UnexpectedResponse {
        expected: "Done".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  //====================================================================================================================
  // Получение информации о системе
  //====================================================================================================================

  /// Возвращает версию протокола API, поддерживаемую сервером.
  ///
  /// # Returns
  /// Строка с версией, например `"9.54"`.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// # Notes
  /// Текущая библиотека тестировалась с версией `9.54`. При использовании других версий
  /// возможны расхождения (открытая спецификация отсутствует).
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// let version = client.protocol_info_get(&session.session_id).await.unwrap();
  /// println!("Protocol version: {}", version);
  /// ```
  #[instrument(skip(self), err, fields(method = "protocol_info_get"))]
  pub async fn protocol_info_get(&self, session_id: &str) -> Result<String> {
    let body = self.api(&ProtocolInfoGet {}).await?;
    match body {
      ResponseBody::ProtocolInfo(info) => Ok(info.version),
      _ => Err(Error::UnexpectedResponse {
        expected: "ProtocolInfo".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает версию базы данных, используемой сервером.
  ///
  /// # Returns
  /// Строка, например `"12.2.0.1"` для Oracle.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "system_server_version_get"))]
  pub async fn system_server_version_get(&self, session_id: &str) -> Result<String> {
    let body = self.api(&SystemServerVersionGet { session_id }).await?;
    match body {
      ResponseBody::ServerInfo(info) => Ok(info.version),
      _ => Err(Error::UnexpectedResponse {
        expected: "ServerInfo".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает подробную информацию о ядре системы.
  ///
  /// # Returns
  /// Структура [`CoreInfo`], содержащая:
  /// - `auditor` – код аудитора (например, "AUD")
  /// - `owner` – владелец (например, "IBS")
  /// - `version` – версия ТЯ (например, "7.6")
  /// - `build` – номер сборки
  /// - `revision` – ревизия
  /// - `as_version` – версия сервера приложений
  /// - `aswar_date` – дата сборки сервера приложений
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "system_core_info_get"))]
  pub async fn system_core_info_get(&self, session_id: &str) -> Result<CoreInfo> {
    let body = self.api(&SystemCoreInfoGet { session_id }).await?;
    match body {
      ResponseBody::CoreInfo(info) => Ok(info),
      _ => Err(Error::UnexpectedResponse {
        expected: "CoreInfo".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает все системные настройки в формате ключ‑значение.
  ///
  /// На серверной стороне (Oracle) это эквивалентно `SELECT name, value FROM IBS.SETTINGS`.
  ///
  /// # Returns
  /// Массив структур [`Setting`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// let settings = client.system_settings_get(&session.session_id).await.unwrap();
  /// for s in settings {
  ///   println!("{} = {:?}", s.name, s.value);
  /// }
  /// ```
  #[instrument(skip(self), err, fields(method = "system_settings_get"))]
  pub async fn system_settings_get(&self, session_id: &str) -> Result<Vec<Setting>> {
    let body = self.api(&SystemSettingsGet { session_id }).await?;
    match body {
      ResponseBody::Settings(settings) => Ok(settings.body),
      _ => Err(Error::UnexpectedResponse {
        expected: "Settings".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает значение конкретной системной настройки по её имени.
  ///
  /// # Arguments
  /// * `name` – имя настройки (например, `"SHOW_SYSTEM_MENU"`).
  ///
  /// # Returns
  /// `Some(String)` – если настройка существует и имеет непустое значение, иначе `None`.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// let value = client.system_setting_get(&session.session_id, "SHOW_SYSTEM_MENU").await.unwrap();
  /// assert_eq!(value, Some("YES".to_string()));
  /// ```
  #[instrument(skip(self), err, fields(method = "system_setting_get"))]
  pub async fn system_setting_get(&self, session_id: &str, name: &str) -> Result<Option<String>> {
    let body = self.api(&SystemSettingGet { session_id, name }).await?;
    match body {
      ResponseBody::Setting(setting) => Ok(setting.value),
      _ => Err(Error::UnexpectedResponse {
        expected: "Setting".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает относительный URL для эндпоинта авторизации.
  ///
  /// # Returns
  /// Строка, например `"/platform2mca/authbasic"`.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "authentication_url_get"))]
  pub async fn authentication_url_get(&self) -> Result<String> {
    let body = self.api(&AuthenticationURLGet {}).await?;
    match body {
      ResponseBody::AuthenticationURL(url) => Ok(url.url),
      _ => Err(Error::UnexpectedResponse {
        expected: "AuthenticationURL".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Проверяет, разрешено ли использование функционала NOVO для текущей сессии.
  ///
  /// # Returns
  /// `true`, если NOVO доступен.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "novo_allowed_check"))]
  pub async fn novo_allowed_check(&self, session_id: &str) -> Result<bool> {
    let body = self.api(&NovoAllowedCheck { session_id }).await?;
    match body {
      ResponseBody::NovoAllowedCheckResult(result) => Ok(result.value),
      _ => Err(Error::UnexpectedResponse {
        expected: "NovoAllowedCheckResult".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Проверяет, включена ли указанная системная опция.
  ///
  /// На серверной стороне (Oracle) это значение берётся из таблицы `IBS.SYSTEM_OPTIONS`.
  ///
  /// # Arguments
  /// * `option_name` – имя опции (например, `"NAV_SKIN_INTERFACE"`).
  ///
  /// # Returns
  /// `true`, если опция включена.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// let enabled = client.system_option_enabled_check(&session.session_id, "NAV_SKIN_INTERFACE").await.unwrap();
  /// println!("NAV_SKIN_INTERFACE enabled: {}", enabled);
  /// ```
  #[instrument(skip(self), err, fields(method = "system_option_enabled_check"))]
  pub async fn system_option_enabled_check(&self, session_id: &str, option_name: &str) -> Result<bool> {
    let body = self
      .api(&SystemOptionEnabledCheck {
        session_id,
        option_name,
      })
      .await?;
    match body {
      ResponseBody::OptionInfo(info) => Ok(info.enabled),
      _ => Err(Error::UnexpectedResponse {
        expected: "OptionInfo".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  //====================================================================================================================
  // Получение информации о пользователе
  //====================================================================================================================

  /// Проверяет, является ли текущий пользователь привилегированным.
  ///
  /// # Returns
  /// `true`, если пользователь имеет привилегии.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "system_user_privileged_get"))]
  pub async fn system_user_privileged_get(&self, session_id: &str) -> Result<bool> {
    let body = self.api(&SystemUserPrivilegedGet { session_id }).await?;
    match body {
      ResponseBody::User(UserContent::Privileged(u)) => Ok(u.is_privileged),
      _ => Err(Error::UnexpectedResponse {
        expected: "User".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает детальную информацию о пользователе.
  ///
  /// На серверной стороне (Oracle) данные берутся из таблицы `IBS.USERS`.
  ///
  /// # Returns
  /// Структура [`User`] с полями:
  /// - `name` – полное имя (ФИО)
  /// - `short_name` – короткое имя в системе
  /// - `properties` – строка параметров, разделённых `|` (например, `|ADMIN|CONTEXT|...`)
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "user_info_get"))]
  pub async fn user_info_get(&self, session_id: &str) -> Result<User> {
    let body = self.api(&UserInfoGet { session_id }).await?;
    match body {
      ResponseBody::User(UserContent::Info(user)) => Ok(user),
      _ => Err(Error::UnexpectedResponse {
        expected: "User".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает значение свойства профиля пользователя по его имени.
  ///
  /// На серверной стороне (Oracle) профиль пользователя определяется из таблицы `IBS.USERS` (колонка `PROPERTIES`),
  /// а значение свойства – из таблицы `ibs.profiles`.
  ///
  /// # Arguments
  /// * `property_name` – имя свойства (например, `"SESSIONS_PER_USER"`).
  ///
  /// # Returns
  /// Значение свойства в виде строки (например, `"UNLIMITED"`).
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// let value = client.user_profile_property_get(&session.session_id, "SESSIONS_PER_USER").await.unwrap();
  /// println!("SESSIONS_PER_USER = {}", value);
  /// ```
  #[instrument(skip(self), err, fields(method = "user_profile_property_get"))]
  pub async fn user_profile_property_get(&self, session_id: &str, property_name: &str) -> Result<String> {
    let body = self
      .api(&UserProfilePropertyGet {
        session_id,
        property_name,
      })
      .await?;
    match body {
      ResponseBody::UserProfileProperty(u) => Ok(u.value),
      _ => Err(Error::UnexpectedResponse {
        expected: "UserProfileProperty".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  // Проверяет, входит ли пользователь в указанную группу.
  ///
  /// На серверной стороне (Oracle) проверка выполняется по таблице `IBS.GROUP_USERS`.
  ///
  /// # Arguments
  /// * `group_id` – идентификатор группы (например, `"ADMIN_GRP"`).
  ///
  /// # Returns
  /// `true`, если пользователь является членом группы.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// ## Examples
  ///
  /// ```rust,ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// let is_admin = client.user_belongs_group_check(session_id, "ADMIN_GRP").await.unwrap();
  /// println!("is_admin = {}", value);
  /// ```
  #[instrument(skip(self), err, fields(method = "user_belongs_group_check"))]
  pub async fn user_belongs_group_check(&self, session_id: &str, group_id: &str) -> Result<bool> {
    let body = self.api(&UserBelongsGroupCheck { session_id, group_id }).await?;
    match body {
      ResponseBody::CheckResult(result) => Ok(result.value),
      _ => Err(Error::UnexpectedResponse {
        expected: "CheckResult".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  //====================================================================================================================
  // Отладка
  //====================================================================================================================

  /// Получает текст из отладочного канала (Pipe) по его имени.
  ///
  /// # Arguments
  /// * `pipe_name` – имя канала (например, полученное из `Session::debug_pipe_name`).
  ///
  /// # Returns
  /// Текст, сгенерированный сервером для этого канала.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "pipe_text_get"))]
  pub async fn pipe_text_get(&self, session_id: &str, pipe_name: &str) -> Result<String> {
    let body = self.api(&PipeTextGet { session_id, pipe_name }).await?;
    match body {
      ResponseBody::PipeText(text) => Ok(text.value),
      _ => Err(Error::UnexpectedResponse {
        expected: "PipeText".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает отладочный текст по направлению.
  ///
  /// # Returns
  /// Отладочная информация в виде строки.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "debug_text_get"))]
  pub async fn debug_text_get(&self, session_id: &str, direction: &str) -> Result<String> {
    let body = self.api(&DebugTextGet { session_id, direction }).await?;
    match body {
      ResponseBody::DebugText(text) => Ok(text.value),
      _ => Err(Error::UnexpectedResponse {
        expected: "DebugText".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  //====================================================================================================================
  // ТБП и типы
  //====================================================================================================================

  /// Возвращает короткое имя ТБП и ключ архива для указанного экземпляра.
  ///
  /// # Arguments
  /// * `object_id` – идентификатор экземпляра
  /// * `base_class_id` – короткое имя базового ТБП (например, `"DOCUMENT"`)
  ///
  /// # Returns
  /// Структура [`ObjectClassAndArchiveKey`] с полями `class_id` (текущий тип экземпляра, например `"MAIN_DOCUM"`) и `archive_key`.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "object_class_and_archive_key_get"))]
  pub async fn object_class_and_archive_key_get(
    &self,
    session_id: &str,
    object_id: i64,
    base_class_id: &str,
  ) -> Result<ObjectClassAndArchiveKey> {
    let body = self
      .api(&ObjectClassAndArchiveKeyGet {
        session_id,
        object_id,
        base_class_id,
      })
      .await?;
    match body {
      ResponseBody::ObjectClassAndArchiveKey(key) => Ok(key),
      _ => Err(Error::UnexpectedResponse {
        expected: "ObjectClassAndArchiveKey".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает список обратных ссылок на указанный экземпляр.
  ///
  /// Обратные ссылки – это объекты, которые ссылаются на данный экземпляр.
  ///
  /// # Arguments
  /// * `object_id` – идентификатор экземпляра
  /// * `class_id` – короткое имя ТБП, к которому принадлежит экземпляр
  ///
  /// # Returns
  /// Вектор структур [`BackwardReference`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "object_backward_references_get"))]
  pub async fn object_backward_references_get(
    &self,
    session_id: &str,
    object_id: i64,
    class_id: &str,
  ) -> Result<Vec<BackwardReference>> {
    let body = self
      .api(&ObjectBackwardReferencesGet {
        session_id,
        object_id,
        class_id,
      })
      .await?;
    match body {
      ResponseBody::BackwardReferences(refs) => Ok(refs.body),
      _ => Err(Error::UnexpectedResponse {
        expected: "BackwardReferences".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает информацию о возможных переходах между состояниями для указанного ТБП.
  ///
  /// # Arguments
  /// * `class_id` – короткое имя ТБП.
  ///
  /// # Returns
  /// Структура [`Transitions`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "class_transitions_get"))]
  pub async fn class_transitions_get(&self, session_id: &str, class_id: &str) -> Result<Vec<Transition>> {
    let body = self.api(&ClassTransitionsGet { session_id, class_id }).await?;
    match body {
      ResponseBody::Transitions(v) => Ok(v.transitions),
      _ => Err(Error::UnexpectedResponse {
        expected: "Transitions".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает список состояний для указанного ТБП.
  ///
  /// # Arguments
  /// * `class_id` – короткое имя ТБП.
  ///
  /// # Returns
  /// Структура [`States`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "class_states_get"))]
  pub async fn class_states_get(&self, session_id: &str, class_id: &str) -> Result<Vec<State>> {
    let body = self.api(&ClassStatesGet { session_id, class_id }).await?;
    match body {
      ResponseBody::States(v) => Ok(v.states),
      _ => Err(Error::UnexpectedResponse {
        expected: "States".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Проверяет, требуется ли указывать `collectionid` для данного ТБП.
  ///
  /// # Arguments
  /// * `class_id` – короткое имя ТБП.
  ///
  /// # Returns
  /// `true`, если `collectionid` обязателен.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "class_need_collection_id_check"))]
  pub async fn class_need_collection_id_check(&self, session_id: &str, class_id: &str) -> Result<bool> {
    let body = self.api(&ClassNeedCollectionIDCheck { session_id, class_id }).await?;
    match body {
      ResponseBody::CheckResult(result) => Ok(result.value),
      _ => Err(Error::UnexpectedResponse {
        expected: "CheckResult".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает список дочерних ТБП для указанного ТБП.
  ///
  /// # Arguments
  /// * `class_id` – короткое имя родительского ТБП.
  ///
  /// # Returns
  /// Структура [`ChildClasses`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "class_children_get"))]
  pub async fn class_children_get(&self, session_id: &str, class_id: &str) -> Result<Vec<ChildClass>> {
    let body = self.api(&ClassChildrenGet { session_id, class_id }).await?;
    match body {
      ResponseBody::ChildClasses(v) => Ok(v.child_classes),
      _ => Err(Error::UnexpectedResponse {
        expected: "ChildClasses".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает детальную информацию о нескольких ТБП/типах.
  ///
  /// # Arguments
  /// * `class_info` – массив коротких имен ТБП.
  ///
  /// # Returns
  /// Вектор структур [`Class`] с полной информацией.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "classes_get"))]
  pub async fn classes_get(&self, session_id: &str, class_info: &[ClassInfo<'_>]) -> Result<Vec<Class>> {
    let body = self.api(&ClassesGet { session_id, class_info }).await?;
    match body {
      ResponseBody::Classes(cls) => Ok(cls.body),
      _ => Err(Error::UnexpectedResponse {
        expected: "Classes".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает детальную информацию о конкретном ТБП.
  ///
  /// # Arguments
  /// * `class_id` – короткое имя ТБП.
  ///
  /// # Returns
  /// `Some(Class)` – если ТБП существует, иначе `None`.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "class_get"))]
  pub async fn class_get(&self, session_id: &str, class_id: &str) -> Result<Option<Class>> {
    let body = self.api(&ClassGet { session_id, class_id }).await?;
    match body {
      ResponseBody::NotFound(_) => Ok(None),
      ResponseBody::Class(cl) => Ok(Some(cl)),
      _ => Err(Error::UnexpectedResponse {
        expected: "Class".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  //====================================================================================================================
  // Операции
  //====================================================================================================================

  /// Возвращает список операций, доступных для указанного ТБП.
  ///
  /// # Arguments
  /// * `class_id` – короткое имя ТБП.
  ///
  /// # Returns
  /// Вектор структур [`Method`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "class_methods_get"))]
  pub async fn class_methods_get(&self, session_id: &str, class_id: &str) -> Result<Vec<Method>> {
    let body = self.api(&ClassMethodsGet { session_id, class_id }).await?;
    match body {
      ResponseBody::Methods(methods) => Ok(methods.body),
      _ => Err(Error::UnexpectedResponse {
        expected: "Methods".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает клиент‑скрипт для указанной операции.
  ///
  /// # Arguments
  /// * `method_id` – идентификатор операции.
  ///
  /// # Returns
  /// `Some(String)` – текст скрипта, если он есть; иначе `None`.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "method_client_script_get"))]
  pub async fn method_client_script_get(&self, session_id: &str, method_id: i64) -> Result<Option<String>> {
    let body = self.api(&MethodClientScriptGet { session_id, method_id }).await?;
    match body {
      ResponseBody::ClientScript(s) if s.text.is_empty() => Ok(None),
      ResponseBody::ClientScript(s) => Ok(Some(s.text)),
      _ => Err(Error::UnexpectedResponse {
        expected: "Validate".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Начинает выполнение операции – открывает форму.
  ///
  /// # Arguments
  /// * `method_id` – идентификатор операции.
  ///
  /// # Returns
  /// Идентификатор открытой формы (`frame_id`), необходимый для последующих вызовов.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "method_begin"))]
  pub async fn method_begin(&self, session_id: &str, method_id: i64) -> Result<i64> {
    let body = self.api(&MethodBegin { session_id, method_id }).await?;
    match body {
      ResponseBody::MethodFrame(MethodFrame {
        frame_id: Some(frame_id),
      }) => Ok(frame_id),
      _ => Err(Error::UnexpectedResponse {
        expected: "MethodFrame".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Завершает выполнение операции – закрывает форму.
  ///
  /// # Arguments
  /// * `frame_id` – идентификатор формы, полученный из [`method_begin`](Client::method_begin).
  ///
  /// # Returns
  /// `Some(i64)` – идентификатор предыдущей открытой формы, если она существовала.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "method_end"))]
  pub async fn method_end(&self, session_id: &str, frame_id: i64) -> Result<Option<i64>> {
    let body = self.api(&MethodEnd { session_id, frame_id }).await?;
    match body {
      ResponseBody::MethodFrame(frame) => Ok(frame.frame_id),
      _ => Err(Error::UnexpectedResponse {
        expected: "MethodFrame".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает список входных параметров (P‑параметров) операции.
  ///
  /// # Arguments
  /// * `method_id` – идентификатор операции.
  ///
  /// # Returns
  /// Вектор структур [`MethodParameter`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "method_parameters_get"))]
  pub async fn method_parameters_get(&self, session_id: &str, method_id: i64) -> Result<Vec<MethodParameter>> {
    let body = self.api(&MethodParametersGet { session_id, method_id }).await?;
    match body {
      ResponseBody::MethodParameters(p) => Ok(p.parameters),
      _ => Err(Error::UnexpectedResponse {
        expected: "MethodParameters".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает список публичных переменных (V‑переменных) операции.
  ///
  /// # Arguments
  /// * `method_id` – идентификатор операции.
  ///
  /// # Returns
  /// Вектор структур [`MethodVariable`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "method_variables_get"))]
  pub async fn method_variables_get(&self, session_id: &str, method_id: i64) -> Result<Vec<MethodVariable>> {
    let body = self.api(&MethodVariablesGet { session_id, method_id }).await?;
    match body {
      ResponseBody::MethodVariables(p) => Ok(p.variables),
      _ => Err(Error::UnexpectedResponse {
        expected: "MethodVariables".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает список элементов управления (controls) на форме операции.
  ///
  /// # Arguments
  /// * `form_id` – идентификатор операции форму которой открываем.
  ///
  /// # Returns
  /// Вектор структур [`Control`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "method_controls_get"))]
  pub async fn method_controls_get(&self, session_id: &str, form_id: i64) -> Result<Vec<Control>> {
    let body = self.api(&MethodControlsGet { session_id, form_id }).await?;
    match body {
      ResponseBody::Controls(c) => Ok(c.controls),
      _ => Err(Error::UnexpectedResponse {
        expected: "Controls".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает группы операций пользователя для указанного ТБП.
  ///
  /// # Arguments
  /// * `class_id` – короткое имя ТБП.
  ///
  /// # Returns
  /// Структура [`MethodsGroups`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "class_methods_groups_user_get"))]
  pub async fn class_methods_groups_user_get(&self, session_id: &str, class_id: &str) -> Result<Vec<MethodsGroup>> {
    let body = self.api(&ClassMethodsGroupsUserGet { session_id, class_id }).await?;
    match body {
      ResponseBody::MethodsGroups(v) => Ok(v.methods_group),
      _ => Err(Error::UnexpectedResponse {
        expected: "MethodsGroups".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Выполняет блок `Validate` операции по умолчанию (при открытии формы).
  ///
  /// # Arguments
  /// * `req` – структура [`MethodValidateDefault`] с параметрами.
  ///
  /// # Returns
  /// Структура [`Validate`], содержащая значения элементов формы и отладочный текст.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "method_validate_default"))]
  pub async fn method_validate_default(&self, req: &MethodValidateDefault<'_>) -> Result<Validate> {
    let body = self.api(req).await?;
    match body {
      ResponseBody::Validate(validate) => Ok(validate),
      _ => Err(Error::UnexpectedResponse {
        expected: "Validate".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Выполняет блок `Validate` операции по событию элемента формы (например, изменение значения).
  ///
  /// # Arguments
  /// * `req` – структура [`MethodValidate`] с параметрами.
  ///
  /// # Returns
  /// Структура [`Validate`], содержащая обновлённые значения элементов и отладочный текст.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "method_validate"))]
  pub async fn method_validate(&self, req: &MethodValidate<'_>) -> Result<Validate> {
    let body = self.api(req).await?;
    match body {
      ResponseBody::Validate(validate) => Ok(validate),
      _ => Err(Error::UnexpectedResponse {
        expected: "Validate".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Выполняет блок `Execute` операции (непосредственное действие).
  ///
  /// # Arguments
  /// * `req` – структура [`MethodExecute`] с параметрами.
  ///
  /// # Returns
  /// Структура [`MethodResult`], содержащая результат выполнения и обновлённые состояния элементов.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "method_execute"))]
  pub async fn method_execute(&self, req: &MethodExecute<'_>) -> Result<MethodResult> {
    let body = self.api(req).await?;
    match body {
      ResponseBody::Result(result) => Ok(result),
      _ => Err(Error::UnexpectedResponse {
        expected: "Result".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  //====================================================================================================================
  // Представления и данные
  //====================================================================================================================

  /// Получает структуру пользовательского меню представлений.
  ///
  /// # Returns
  /// Структура [`UserMenu`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "user_menu_get"))]
  pub async fn user_menu_get(&self, session_id: &str) -> Result<Vec<UserMenuItem>> {
    let body = self.api(&UserMenuGet { session_id }).await?;
    match body {
      ResponseBody::UserMenu(menu) => Ok(menu.user_menu_items),
      _ => Err(Error::UnexpectedResponse {
        expected: "UserMenu".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает данные представления (табличные данные) с возможностью отмены.
  ///
  /// # Arguments
  /// * `req` – структура [`ViewDataGetCancelable`] с параметрами фильтрации.
  ///
  /// # Returns
  /// Массив строк [`Row`], каждая строка содержит набор значений колонок.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "view_data_get_cancelable"))]
  pub async fn view_data_get_cancelable(&self, req: &ViewDataGetCancelable<'_>) -> Result<Vec<Row>> {
    let body = self.api(req).await?;
    match body {
      ResponseBody::ViewData(data) => Ok(data.row),
      _ => Err(Error::UnexpectedResponse {
        expected: "ViewData".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает список колонок для указанного представления.
  ///
  /// # Arguments
  /// * `view_id` – идентификатор представления.
  ///
  /// # Returns
  /// Вектор структур [`Column`] с описанием каждой колонки.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "view_columns_get"))]
  pub async fn view_columns_get(&self, session_id: &str, view_id: i64) -> Result<Vec<Column>> {
    let body = self.api(&ViewColumnsGet { session_id, view_id }).await?;
    match body {
      ResponseBody::Columns(columns) => Ok(columns.body),
      _ => Err(Error::UnexpectedResponse {
        expected: "Columns".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Возвращает список представлений, доступных для указанного ТБП.
  ///
  /// # Arguments
  /// * `class_id` – короткое имя ТБП.
  ///
  /// # Returns
  /// Вектор структур [`View`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "class_views_get"))]
  pub async fn class_views_get(&self, session_id: &str, class_id: &str) -> Result<Vec<View>> {
    let body = self.api(&ClassViewsGet { session_id, class_id }).await?;
    match body {
      ResponseBody::Views(views) => Ok(views.body),
      _ => Err(Error::UnexpectedResponse {
        expected: "Views".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  //====================================================================================================================
  // Навигация, справочники и меню
  //====================================================================================================================

  /// Получает список справочников, доступных пользователю.
  ///
  /// # Returns
  /// Вектор структур [`Class`] (справочники являются ТБП с определёнными атрибутами).
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "guides_get"))]
  pub async fn guides_get(&self, session_id: &str) -> Result<Vec<Class>> {
    let body = self.api(&GuidesGet { session_id }).await?;
    match body {
      ResponseBody::Guides(guides) => Ok(guides.body),
      _ => Err(Error::UnexpectedResponse {
        expected: "Guides".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает список групп справочников.
  ///
  /// # Returns
  /// Вектор структур [`GuidesGroup`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "guides_groups_get"))]
  pub async fn guides_groups_get(&self, session_id: &str) -> Result<Vec<GuidesGroup>> {
    let body = self.api(&GuidesGroupsGet { session_id }).await?;
    match body {
      ResponseBody::GuidesGroups(groups) => Ok(groups.body),
      _ => Err(Error::UnexpectedResponse {
        expected: "GuidesGroups".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Получает список всех типов системы.
  ///
  /// # Returns
  /// Вектор структур [`Class`].
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  #[instrument(skip(self), err, fields(method = "types_get"))]
  pub async fn types_get(&self, session_id: &str) -> Result<Vec<Class>> {
    let body = self.api(&TypesGet { session_id }).await?;
    match body {
      ResponseBody::Types(types) => Ok(types.body),
      _ => Err(Error::UnexpectedResponse {
        expected: "Types".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  //====================================================================================================================
  // Блокировки
  //====================================================================================================================

  /// Блокирует один или несколько экземпляров.
  ///
  /// # Arguments
  /// * `objects` – массив структур [`Object`], содержащих `id` и `class_id`.
  ///
  /// # Returns
  /// `None` – если блокировка успешна; иначе `Some(String)` с сообщением о причине неудачи.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # use as2mca_api::requests::Object;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// let objects = [Object { id: 123, class_id: "CL_PRIV" }];
  /// let error_msg = client.objects_lock(&session.session_id, &objects).await.unwrap();
  /// if let Some(msg) = error_msg {
  ///   eprintln!("Lock failed: {}", msg);
  /// }
  /// ```
  #[instrument(skip(self), err, fields(method = "objects_lock"))]
  pub async fn objects_lock(&self, session_id: &str, objects: &[Object<'_>]) -> Result<Option<String>> {
    let body = self.api(&ObjectsLock { session_id, objects }).await?;
    match body {
      ResponseBody::LockResult(r) => Ok(r.message),
      _ => Err(Error::UnexpectedResponse {
        expected: "LockResult".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Разблокирует экземпляры (снять все блокировки или только текущей сессии).
  ///
  /// # Arguments
  /// * `clear_all_locks` – если `true`, снимаются все блокировки (в том числе других сессий);
  ///   если `false`, снимаются только блокировки текущей сессии.
  ///
  /// # Errors
  /// Стандартные ошибки API и сетевые ошибки.
  ///
  /// # Examples
  /// ```ignore
  /// # use as2mca_api::client::Client;
  /// # let client = Client::new("http://localhost:3000/platform2mca").unwrap();
  /// # let session = client.session_init(None).await.unwrap();
  /// client.objects_unlock(&session.session_id, false).await.unwrap();
  /// ```
  #[instrument(skip(self), err, fields(method = "objects_unlock"))]
  pub async fn objects_unlock(&self, session_id: &str, clear_all_locks: bool) -> Result<()> {
    let body = self
      .api(&ObjectsUnlock {
        session_id,
        clear_all_locks,
      })
      .await?;
    match body {
      ResponseBody::Done(_) => Ok(()),
      _ => Err(Error::UnexpectedResponse {
        expected: "Done".to_string(),
        actual: format!("{body:?}"),
      }),
    }
  }

  /// Формирует полный URL для заданного относительного пути.
  ///
  /// # Arguments
  /// * `path` – относительный путь (например, `"/authbasic"`).
  ///
  /// # Returns
  /// Полный URL в виде `Url`.
  ///
  /// # Errors
  /// [`Error::UrlParseError`], если объединение с базовым URL не удалось.
  #[inline]
  pub(crate) fn endpoint(&self, path: &str) -> Result<Url> {
    self
      .base_url
      .join(path.trim_start_matches('/'))
      .map_err(|e| Error::UrlParseError(e.to_string()))
  }

  /// Выполняет API‑запрос с сериализацией тела в XML и десериализацией ответа.
  ///
  /// # Type parameters
  /// * `T` – тип структуры запроса, реализующий `serde::Serialize`.
  ///
  /// # Arguments
  /// * `body` – ссылка на структуру запроса.
  ///
  /// # Returns
  /// Тело ответа в виде [`ResponseBody`].
  ///
  /// # Errors
  /// - [`Error::Http`] – при сетевых проблемах.
  /// - [`Error::XmlSerializeError`] – если тело запроса не удалось сериализовать.
  /// - [`Error::XmlDeserializeError`] – если ответ не удалось разобрать.
  /// - [`Error::Api`] – если сервер вернул ошибку.
  /// - [`Error::UnexpectedResponse`] – если структура ответа не соответствует ожидаемой.
  pub(crate) async fn api<T>(&self, body: &T) -> Result<ResponseBody>
  where
    T: serde::Serialize + Sync,
  {
    let url = self.endpoint("/api")?;

    let xml_body = quick_xml::se::to_string(&Request { body })?;

    let mut body_bytes = Vec::with_capacity(XML_HEADER.len() + xml_body.len());
    body_bytes.extend_from_slice(XML_HEADER.as_bytes());
    body_bytes.extend_from_slice(xml_body.as_bytes());

    tracing::trace!(
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

    tracing::trace!(
      len = response_bytes.len(),
      response = %String::from_utf8_lossy(&response_bytes),
      "<- finished processing request"
    );

    let parsed: Response = quick_xml::de::from_reader(response_bytes.as_ref())?;
    match parsed.body {
      ResponseBody::Error(err) => Err(Error::Api {
        status_code,
        message: err.text,
        details: err.body.text,
      }),
      body => Ok(body),
    }
  }
}
