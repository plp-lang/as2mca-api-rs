//! Модуль, содержащий все структуры, используемые для формирования XML‑тел запросов к API.
//!
//! Каждая структура соответствует одному методу API и сериализуется в XML с помощью `serde`.
//! Общим контейнером для всех запросов служит структура [`Request<T>`], которая оборачивает тело запроса.
//!
//! # Пример использования
//! ```no_run
//! use as2mca_api::requests::{Request, SystemServerVersionGet};
//!
//! let req = SystemServerVersionGet { session_id: "abc123" };
//! let xml = quick_xml::se::to_string(&Request { body: req }).unwrap();
//! println!("{}", xml);
//! ```
//!
//! # Примечание
//! Все структуры реализуют `Serialize` и рассчитаны на использование внутри [`crate::client::Client`].
//! Ручная сериализация обычно не требуется.

use serde::{Deserialize, Serialize};

use crate::serde_helpers::{comma_separated_numbers, unwrap_list};

pub const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;

/// Базовая обертка для любого XML-запроса к API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Request")]
pub struct Request<T> {
  #[serde(rename = "$value")]
  pub body: T,
}

//====================================================================================================================
// Сессия и авторизация
//====================================================================================================================

/// Запрос на инициализацию (активацию) сессии.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionInit {
  #[serde(rename = "@AliveActiveSession", skip_serializing_if = "Option::is_none")]
  /// Флаг, указывающий, следует ли поддерживать активную сессию
  pub alive_active_session: Option<bool>,
}

/// Запрос на деактивацию (завершение) сессии.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Disconnect<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

/// Запрос на получение URL для авторизации.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationURLGet {}

//======================================================================================================================
// Информация о системе
//======================================================================================================================

/// Запрос версии протокола API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolInfoGet {}

/// Запрос версии сервера приложений.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemServerVersionGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

/// Запрос информации о ядре системы.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemCoreInfoGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

/// Запрос всех системных настроек.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemSettingsGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

/// Запрос конкретной системной настройки по имени.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemSettingGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@Name")]
  pub name: &'a str,
}

/// Запрос проверки доступности NOVO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NovoAllowedCheck<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

/// Запрос проверки включения системной опции.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemOptionEnabledCheck<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@OptionName")]
  pub option_name: &'a str,
}

/// Установка информации о сетевом окружении клиента.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkInformationSet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ClientName")]
  pub client_name: &'a str,
  #[serde(rename = "@ClientIP")]
  pub client_ip: &'a str,
  #[serde(rename = "@ClientUser")]
  pub client_user: &'a str,
  #[serde(rename = "@ModuleName")]
  pub module_name: &'a str,
}

/// Установка MAC и IP адресов клиента.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemNetAddressSet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@MACAddress")]
  pub mac_address: &'a str,
  #[serde(rename = "@IPAddress")]
  pub ip_address: &'a str,
}

//======================================================================================================================
// Информация о пользователе
//======================================================================================================================

/// Запрос базовой информации о пользователе.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfoGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

/// Запрос проверки привилегий пользователя.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemUserPrivilegedGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

/// Запрос свойства профиля пользователя.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfilePropertyGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@PropertyName")]
  pub property_name: &'a str,
}

/// Запрос проверки вхождения пользователя в группу.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBelongsGroupCheck<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@GroupID")]
  pub group_id: &'a str,
}

//====================================================================================================================
// Отладка
//====================================================================================================================

/// Запрос текста из отладочного канала (Pipe).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipeTextGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@PipeName")]
  pub pipe_name: &'a str,
}

/// Запрос отладочного текста.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugTextGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@Direction")]
  pub direction: &'a str,
}

//======================================================================================================================
// ТБП и их экземпляры
//======================================================================================================================

/// Запрос ТБП и ключа архива для экземпляра.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectClassAndArchiveKeyGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
  #[serde(rename = "@BaseClassID")]
  pub base_class_id: &'a str,
}

/// Запрос обратных ссылок на экземпляр.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectBackwardReferencesGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

/// Запрос переходов состояний ТБП.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassTransitionsGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

/// Запрос состояний ТБП.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassStatesGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

/// Запрос проверки необходимости `CollectionID` для ТБП.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassNeedCollectionIDCheck<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

/// Запрос дочерних ТБП.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassChildrenGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

/// Запрос на получения списка ТБП.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ClassesGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "$value")]
  pub class_info: &'a [ClassInfo<'a>],
}

/// Информация о ТБП для запроса списка ТБП.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassInfo<'a> {
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

/// Запрос на получения информации об ТБП
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

//======================================================================================================================
// Операции
//======================================================================================================================

/// Запрос операций ТБП.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassMethodsGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

/// Запрос на открытие формы операции
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodBegin<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

/// Запрос списка входных параметров операции
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodParametersGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

/// Запрос списка публичных переменных операции
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodVariablesGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

/// Запрос списка элементов формы операции
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodControlsGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@FormID")]
  pub form_id: i64,
}

/// Запрос на получение клиент-скрипта операции
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodClientScriptGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
}

/// Запрос вызова блока `Validate` операции (по умолчанию, при открытии формы).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct MethodValidateDefault<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@Info")]
  pub info: &'a str,
  #[serde(rename = "@DoCommit")]
  pub do_commit: bool,
  #[serde(rename = "@ObjectID", with = "comma_separated_numbers")]
  pub object_id: &'a [i64],
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
  #[serde(rename = "@DebugLevel")]
  pub debug_level: u8,
  #[serde(rename = "@IsCalledFromAnotherMethod")]
  pub is_called_from_another_method: bool,
  #[serde(rename = "@ReadOnly")]
  pub read_only: bool,
  #[serde(rename = "@LockObjectClassID", skip_serializing_if = "Option::is_none")]
  pub lock_object_class_id: Option<&'a str>,
  #[serde(rename = "@GetDebugText")]
  pub get_debug_text: bool,
  #[serde(rename = "@OptimizedGridUpdates")]
  pub optimized_grid_updates: bool,
}

impl Default for MethodValidateDefault<'_> {
  fn default() -> Self {
    Self {
      session_id: Default::default(),
      method_id: Default::default(),
      class_id: Default::default(),
      info: "",
      do_commit: true,
      object_id: &[],
      debug_level: 0,
      is_called_from_another_method: true,
      lock_object_class_id: None,
      read_only: false,
      get_debug_text: true,
      optimized_grid_updates: true,
    }
  }
}

/// Запрос на вызов блока `Validate` операции при событии элемента формы.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MethodValidate<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@Type")]
  pub r#type: ValidateType,
  #[serde(rename = "@Info")]
  pub info: &'a str,
  #[serde(rename = "@DoCommit")]
  pub do_commit: bool,
  #[serde(rename = "@GetDebugText")]
  pub get_debug_text: bool,
  #[serde(rename = "@OptimizedGridUpdates")]
  pub optimized_grid_updates: bool,
  #[serde(rename = "ControlsStates", with = "unwrap_list")]
  pub controls_states: &'a [ControlState<'a>],
  #[serde(rename = "PLPCallParameters", with = "unwrap_list")]
  pub plpcall_parameters: &'a [PLPCallParameter<'a>],
}

impl Default for MethodValidate<'_> {
  fn default() -> Self {
    Self {
      session_id: Default::default(),
      method_id: Default::default(),
      info: Default::default(),
      r#type: ValidateType::Validate,
      controls_states: &[],
      plpcall_parameters: &[],
      do_commit: true,
      get_debug_text: true,
      optimized_grid_updates: true,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValidateType {
  Validate,
}

/// Запрос на вызов блока `Execute` операции.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MethodExecute<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@DoCommit")]
  pub do_commit: bool,
  #[serde(rename = "@OptimizedGridUpdates")]
  pub optimized_grid_updates: bool,
  #[serde(rename = "ControlsStates", with = "unwrap_list")]
  pub controls_states: &'a [ControlState<'a>],
  #[serde(rename = "PLPCallParameters", with = "unwrap_list")]
  pub plpcall_parameters: &'a [PLPCallParameter<'a>],
}

impl Default for MethodExecute<'_> {
  fn default() -> Self {
    Self {
      session_id: Default::default(),
      method_id: Default::default(),
      controls_states: &[],
      plpcall_parameters: &[],
      do_commit: true,
      optimized_grid_updates: true,
    }
  }
}

/// Состояние элемента управления на форме.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlState<'a> {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Value")]
  pub value: &'a str,
}

/// Константа для PLP-вызова.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PLPConstant<'a> {
  #[serde(rename = "@Value")]
  pub value: &'a str,
}

/// Переменная для PLP-вызова.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PLPVariable<'a> {
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@Name")]
  pub name: &'a str,
}

/// Параметр для PLP-вызова.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PLPParameter<'a> {
  #[serde(rename = "@MethodID")]
  pub method_id: i64,
  #[serde(rename = "@Name")]
  pub name: &'a str,
}

/// Объединённый тип сущности для PLP вызова.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum PLPEntity<'a> {
  PLPConstant(PLPConstant<'a>),
  PLPVariable(PLPVariable<'a>),
  PLPParameter(PLPParameter<'a>),
}

/// Параметр PLP-вызова (источник и цель).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PLPCallParameter<'a> {
  #[serde(rename = "TargetPLPCallItem", with = "unwrap_list")]
  pub target: &'a [PLPEntity<'a>],
  #[serde(rename = "SourcePLPCallItem", with = "unwrap_list")]
  pub source: &'a [PLPEntity<'a>],
}

/// Запрос на завершение выполнения операции (закрытие формы).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodEnd<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@FrameID")]
  pub frame_id: i64,
}

//======================================================================================================================
// Представления и данные
//======================================================================================================================

/// Запрос данных представления.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ViewDataGetCancelable<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ViewShortName")]
  pub view_short_name: &'a str,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
  #[serde(rename = "@Hint")]
  pub hint: &'a str,
  #[serde(rename = "@AllowTimestampMilliseconds")]
  pub allow_timestamp_milliseconds: bool,
  #[serde(rename = "@RowsLimit", skip_serializing_if = "Option::is_none")]
  pub rows_limit: Option<i64>,
  #[serde(rename = "AdditionalFilterBind", skip_serializing_if = "Option::is_none")]
  pub additional_filter_bind: Option<AdditionalFilterBind<'a>>,
  #[serde(rename = "ObjectFilter", skip_serializing_if = "Option::is_none")]
  pub object_filter: Option<ObjectFilter>,
  #[serde(rename = "UserFilter", skip_serializing_if = "Option::is_none")]
  pub user_filter: Option<UserFilter<'a>>,
}

impl Default for ViewDataGetCancelable<'_> {
  fn default() -> Self {
    Self {
      session_id: Default::default(),
      view_short_name: Default::default(),
      class_id: Default::default(),
      hint: "FIRST_ROWS",
      allow_timestamp_milliseconds: true,
      rows_limit: Some(10),
      additional_filter_bind: None,
      object_filter: None,
      user_filter: None,
    }
  }
}

/// Дополнительная привязка фильтра для представления.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdditionalFilterBind<'a> {
  #[serde(rename = "@Clause")]
  pub clause: &'a str,
}

/// Фильтр по идентификатору экземпляра.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectFilter {
  #[serde(rename = "@ObjectID")]
  pub object_id: i64,
}

/// Простой фильтр для представления.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimpleFilter<'a> {
  #[serde(rename = "@ColumnName")]
  pub column_name: &'a str,
  #[serde(rename = "@Operator")]
  pub operator: &'a str,
  #[serde(rename = "@Value")]
  pub value: Option<&'a str>,
}

/// Регистронезависимый фильтр для представления.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseInsensitiveFilter<'a> {
  #[serde(rename = "@ColumnName")]
  pub column_name: &'a str,
  #[serde(rename = "@Operator")]
  pub operator: &'a str,
  #[serde(rename = "@Value")]
  pub value: Option<&'a str>,
}

/// Объединённый тип фильтра для представления.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Filter<'a> {
  /// Логический фильтр "AND".
  #[serde(rename = "AND")]
  And {
    #[serde(rename = "$value")]
    filters: &'a [Self],
  },
  /// Логический фильтр "OR".
  #[serde(rename = "OR")]
  Or {
    #[serde(rename = "$value")]
    filters: &'a [Self],
  },
  #[serde(untagged)]
  Simple(&'a SimpleFilter<'a>),
  #[serde(untagged)]
  CaseInsensitive(&'a CaseInsensitiveFilter<'a>),
}

/// Пользовательский фильтр для представления.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct UserFilter<'a> {
  #[serde(rename = "@ExtraFilter", skip_serializing_if = "Option::is_none")]
  pub extra_filter: Option<&'a str>,
  #[serde(rename = "$value")]
  pub filters: &'a [Filter<'a>],
}

/// Запрос колонок представления.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewColumnsGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ViewID")]
  pub view_id: i64,
}

/// Запрос представлений ТБП.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassViewsGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

//======================================================================================================================
// Навигация, справочники и меню
//======================================================================================================================

/// Запрос списка справочников.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidesGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

/// Запрос групп справочников.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidesGroupsGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

/// Запрос списка всех ТБП (не справочников) системы.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypesGet<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
}

//======================================================================================================================
// Блокировки
//======================================================================================================================

// Описание экземпляра для блокировки.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Object<'a> {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@ClassID")]
  pub class_id: &'a str,
}

/// Запрос на блокировку одного или нескольких экземпляров.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ObjectsLock<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "$value")]
  pub objects: &'a [Object<'a>],
}

/// Запрос на разблокировку экземпляров.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectsUnlock<'a> {
  #[serde(rename = "@SessionID")]
  pub session_id: &'a str,
  #[serde(rename = "@ClearAllLocks", skip_serializing_if = "Option::is_none")]
  pub clear_all_locks: Option<bool>,
}
