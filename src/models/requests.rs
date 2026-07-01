use core::fmt;

use serde::{Deserialize, Serialize};

pub const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;

/// Базовая обертка для любого XML-запроса к API.
#[derive(Debug, Serialize)]
#[serde(rename = "Request")]
pub struct Request<T> {
  #[serde(rename = "$value")]
  pub body: T,
}

//====================================================================================================================
// Сессия и авторизация
//====================================================================================================================

/// Запрос на инициализацию (активацию) сессии.
#[derive(Debug, Serialize)]
pub struct SessionInit {
  #[serde(rename = "@AliveActiveSession", skip_serializing_if = "Option::is_none")]
  pub alive_active_session: Option<bool>,
}

/// Запрос на деактивацию (завершение) сессии.
#[derive(Debug, Serialize)]
pub struct Disconnect {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос на получение URL для авторизации.
#[derive(Debug, Serialize)]
pub struct AuthenticationURLGet {}

/// Запрос на авторизацию.
#[derive(Debug, Clone)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(String);

impl SessionId {
  #[must_use]
  pub const fn new(id: String) -> Self {
    Self(id)
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for SessionId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<String> for SessionId {
  fn from(s: String) -> Self {
    Self::new(s)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DebugPipeName(String);

impl DebugPipeName {
  #[must_use]
  pub const fn new(id: String) -> Self {
    Self(id)
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for DebugPipeName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<String> for DebugPipeName {
  fn from(s: String) -> Self {
    Self::new(s)
  }
}

//====================================================================================================================
// Информация о системе
//====================================================================================================================

/// Запрос версии протокола API.
#[derive(Debug, Serialize)]
pub struct ProtocolInfoGet {}

/// Запрос версии сервера приложений.
#[derive(Debug, Serialize)]
pub struct SystemServerVersionGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос информации о ядре системы.
#[derive(Debug, Serialize)]
pub struct SystemCoreInfoGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос всех системных настроек.
#[derive(Debug, Serialize)]
pub struct SystemSettingsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос конкретной системной настройки по имени.
#[derive(Debug, Serialize, Clone)]
pub struct SystemSettingGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@Name")]
  pub name: String,
}

/// Запрос проверки доступности NOVO.
#[derive(Debug, Serialize, Clone)]
pub struct NovoAllowedCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос проверки включения системной опции.
#[derive(Debug, Serialize, Clone)]
pub struct SystemOptionEnabledCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@OptionName")]
  pub option_name: String,
}

/// Установка информации о сетевом окружении клиента.
#[derive(Debug, Serialize, Clone)]
pub struct NetworkInformationSet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClientName")]
  pub client_name: String,
  #[serde(rename = "@ClientIP")]
  pub client_ip: String,
  #[serde(rename = "@ClientUser")]
  pub client_user: String,
  #[serde(rename = "@ModuleName")]
  pub module_name: String,
}

/// Установка MAC и IP адресов клиента.
#[derive(Debug, Serialize, Clone)]
pub struct SystemNetAddressSet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MACAddress")]
  pub mac_address: String,
  #[serde(rename = "@IPAddress")]
  pub ip_address: String,
}

//====================================================================================================================
// Информация о пользователе
//====================================================================================================================

/// Запрос базовой информации о пользователе.
#[derive(Debug, Serialize)]
pub struct UserInfoGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос проверки привилегий пользователя.
#[derive(Debug, Serialize, Clone)]
pub struct SystemUserPrivilegedGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос свойства профиля пользователя.
#[derive(Debug, Serialize, Clone)]
pub struct UserProfilePropertyGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@PropertyName")]
  pub property_name: String,
}

/// Запрос проверки вхождения пользователя в группу.
#[derive(Debug, Serialize, Clone)]
pub struct UserBelongsGroupCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@GroupID")]
  pub group_id: String,
}

//====================================================================================================================
// Отладка
//====================================================================================================================

/// Запрос текста из отладочного канала (Pipe).
#[derive(Debug, Serialize, Clone)]
pub struct PipeTextGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@PipeName")]
  pub pipe_name: DebugPipeName,
}

/// Запрос отладочного текста.
#[derive(Debug, Serialize, Clone)]
pub struct DebugTextGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@Direction")]
  pub direction: String,
}

//======================================================================================================================
// ТБП и их экземпляры
//======================================================================================================================

/// Запрос ТБП и ключа архива для экземпляра.
#[derive(Debug, Serialize, Clone)]
pub struct ObjectClassAndArchiveKeyGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
  #[serde(rename = "@BaseClassID")]
  pub base_class_id: String,
}

/// Запрос обратных ссылок на экземпляр.
#[derive(Debug, Serialize, Clone)]
pub struct ObjectBackwardReferencesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос переходов состояний ТБП.
#[derive(Debug, Serialize, Clone)]
pub struct ClassTransitionsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос состояний ТБП.
#[derive(Debug, Serialize, Clone)]
pub struct ClassStatesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос проверки необходимости `CollectionID` для ТБП.
#[derive(Debug, Serialize, Clone)]
pub struct ClassNeedCollectionIDCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос дочерних ТБП.
#[derive(Debug, Serialize, Clone)]
pub struct ClassChildrenGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос на получения списка типов/ТБП.
#[derive(Debug, Serialize, Clone)]
pub struct ClassesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "$value")]
  pub class_info: Vec<ClassInfo>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClassInfo {
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

//======================================================================================================================
// Операции
//======================================================================================================================

/// Запрос операций ТБП.
#[derive(Debug, Serialize, Clone)]
pub struct ClassMethodsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос на подготовку операции к выполнению
#[derive(Debug, Serialize, Clone)]
pub struct MethodBegin {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

/// Запрос списка входных параметров операции
#[derive(Debug, Serialize, Clone)]
pub struct MethodParametersGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

/// Запрос списка публичных переменных операции
#[derive(Debug, Serialize, Clone)]
pub struct MethodVariablesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

/// Запрос списка элементов формы операции
#[derive(Debug, Serialize, Clone)]
pub struct MethodControlsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@FormID")]
  pub form_id: i64,
}

/// Запрос групп операций пользователя для ТБП.
#[derive(Debug, Serialize, Clone)]
pub struct ClassMethodsGroupsUserGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

/// Запрос вызова блока `Validate` операции.
#[derive(Debug, Serialize, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct MethodValidateDefault {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@Info")]
  pub info: String,
  #[serde(rename = "@DoCommit")]
  pub do_commit: bool,
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@DebugLevel")]
  pub debug_level: u8,
  #[serde(rename = "@IsCalledFromAnotherMethod")]
  pub is_called_from_another_method: bool,
  #[serde(rename = "@ReadOnly")]
  pub read_only: bool,
  #[serde(rename = "@GetDebugText")]
  pub get_debug_text: bool,
}

//====================================================================================================================
// Представления и данные
//====================================================================================================================

/// Запрос данных представления.
#[derive(Debug, Serialize, Clone)]
pub struct ViewDataGetCancelable {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ViewShortName")]
  pub view_short_name: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@Hint")]
  pub hint: String,
  #[serde(rename = "@AllowTimestampMilliseconds")]
  pub allow_timestamp_milliseconds: bool,
  #[serde(rename = "@RowsLimit", skip_serializing_if = "Option::is_none")]
  pub rows_limit: Option<i64>,
  #[serde(rename = "$value")]
  pub body: Option<ObjectFilter>,
}

/// Фильтр экземпляра внутри запроса данных представления.
#[derive(Debug, Serialize, Clone)]
pub struct ObjectFilter {
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
}

/// Запрос колонок представления.
#[derive(Debug, Serialize, Clone)]
pub struct ViewColumnsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ViewID")]
  pub view_id: i64,
}

/// Запрос представлений ТБП.
#[derive(Debug, Serialize, Clone)]
pub struct ClassViewsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

//====================================================================================================================
// Навигация, справочники и меню
//====================================================================================================================

/// Запрос пользовательского меню.
#[derive(Debug, Serialize, Clone)]
pub struct UserMenuGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос списка справочников.
#[derive(Debug, Serialize, Clone)]
pub struct GuidesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос групп справочников.
#[derive(Debug, Serialize, Clone)]
pub struct GuidesGroupsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

/// Запрос списка всех ТБП (не справочников) системы.
#[derive(Debug, Serialize, Clone)]
pub struct TypesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

//====================================================================================================================
// Блокировки
//====================================================================================================================

/// Запрос на блокировку экземпляра
#[derive(Debug, Serialize, Clone)]
pub struct ObjectsLock {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "$value")]
  pub objects: Vec<Object>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Object {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}
