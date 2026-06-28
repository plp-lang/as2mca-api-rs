use crate::models::{
  requests::{DebugPipeName, SessionId},
  utils::{Flags, bool_as_bool, bool_as_str, option_bool_as_str},
};
use serde::{Deserialize, Serialize};

/// Базовая обертка XML-ответа от сервера.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "Response")]
pub struct Response<T> {
  #[serde(rename = "$value")]
  pub body: ResponseBody<T>,
}

/// Тело ответа: либо успешные данные, либо ошибка.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ResponseBody<T> {
  Ok(T),
  Error(Error),
}

/// Пустой ответ, подтверждающий успешное выполнение действия (например, отключение сессии).
#[derive(Debug, Deserialize, Clone)]
pub struct Done {}

/// Структура описания ошибки API.
#[derive(Debug, Deserialize, Clone)]
pub struct Error {
  #[serde(rename = "@Text")]
  pub text: String,
  #[serde(rename = "ServerErrorInfo")]
  pub body: ServerErrorInfo,
}

/// Детали серверной ошибки.
#[derive(Debug, Deserialize, Clone)]
pub struct ServerErrorInfo {
  #[serde(rename = "@Text")]
  pub text: String,
}

//======================================================================================================================
// Сессия и авторизация
//======================================================================================================================

/// Ответ с данными успешно инициализированной сессии.
#[derive(Debug, Deserialize, Clone)]
pub struct Session {
  #[serde(rename = "@ID")]
  pub session_id: SessionId,
  #[serde(rename = "@DebugPipeName")]
  pub debug_pipe_name: DebugPipeName,
}

/// Ответ с URL для авторизации.
#[derive(Debug, Deserialize, Clone)]
pub struct AuthenticationURL {
  #[serde(rename = "@URL")]
  pub url: String,
}

//======================================================================================================================
// Информация о системе
//======================================================================================================================

/// Информация о версии протокола.
#[derive(Debug, Deserialize, Clone)]
pub struct ProtocolInfo {
  #[serde(rename = "@Version")]
  pub version: String,
}

/// Информация о версии сервера.
#[derive(Debug, Deserialize, Clone)]
pub struct ServerInfo {
  #[serde(rename = "@Version")]
  pub version: String,
}

/// Информация о ядре системы (ТЯ).
#[derive(Debug, Deserialize, Clone)]
pub struct CoreInfo {
  #[serde(rename = "@Auditor")]
  pub auditor: String,
  #[serde(rename = "@Owner")]
  pub owner: String,
  #[serde(rename = "@Version")]
  pub version: String,
  #[serde(rename = "@Build")]
  pub build: String,
  #[serde(rename = "@Revision")]
  pub revision: String,
  #[serde(rename = "@ASVersion")]
  pub as_version: String,
  #[serde(rename = "@ASWARDate")]
  pub aswar_date: String,
}

/// Список системных настроек.
#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
  #[serde(default, rename = "$value")]
  pub body: Vec<Setting>,
}

/// Конкретная системная настройка (ключ-значение).
#[derive(Debug, Deserialize, Clone)]
pub struct Setting {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Value")]
  pub value: Option<String>,
}

/// Результат проверки доступности NOVO.
#[derive(Debug, Deserialize, Clone)]
pub struct NovoAllowedCheckResult {
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Информация о включенности системной опции.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "User")]
pub struct OptionInfo {
  #[serde(rename = "@Enabled", with = "bool_as_bool")]
  pub enabled: bool,
}

//======================================================================================================================
// Информация о пользователе
//======================================================================================================================

/// Базовая информация о пользователе.
#[derive(Debug, Deserialize, Clone)]
pub struct User {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
}

/// Информация о привилегиях пользователя.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "User")]
pub struct UserPrivileged {
  #[serde(rename = "@IsPrivileged", with = "bool_as_bool")]
  pub is_privileged: bool,
}

/// Значение свойства профиля пользователя.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "User")]
pub struct UserProfileProperty {
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Универсальный результат проверки (например, вхождения в группу).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename = "User")]
pub struct CheckResult {
  #[serde(rename = "@Value")]
  pub value: String,
}

//======================================================================================================================
// Отладка
//======================================================================================================================

/// Текст из отладочного канала (Pipe).
#[derive(Debug, Deserialize, Clone)]
pub struct PipeText {
  #[serde(default, rename = "@Value")]
  pub value: String,
}

/// Отладочный текст.
#[derive(Debug, Deserialize, Clone)]
pub struct DebugText {
  #[serde(default, rename = "@Value")]
  pub value: String,
}

//======================================================================================================================
// ТБП и их экземпляры
//======================================================================================================================

/// Идентификатор ТБП и ключ архива экземпляра.
#[derive(Debug, Deserialize, Clone)]
pub struct ObjectClassAndArchiveKey {
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@ArchiveKey")]
  pub archive_key: String,
}

/// Список обратных ссылок на экземпляр.
#[derive(Debug, Deserialize, Clone)]
pub struct BackwardReferences {
  #[serde(default, rename = "$value")]
  pub body: Vec<BackwardReference>,
}

/// Описание обратной ссылки.
#[derive(Debug, Deserialize, Clone)]
pub struct BackwardReference {
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@ClassName")]
  pub class_name: String,
  #[serde(rename = "@Qual")]
  pub qual: String,
  #[serde(rename = "@QualName")]
  pub qual_name: String,
}

/// Переходы состояний ТБП.
// TODO
#[derive(Debug, Deserialize, Clone)]
pub struct Transitions {}

/// Состояния ТБП.
// TODO
#[derive(Debug, Deserialize, Clone)]
pub struct States {}

/// Список операций ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct Methods {
  #[serde(default, rename = "$value")]
  pub body: Vec<Method>,
}

/// Описание операций ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct Method {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@Type")]
  pub r#type: String,
  #[serde(rename = "@FormClassID")]
  pub form_class_id: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
  #[serde(rename = "@ScriptID")]
  pub script_id: String,
  #[serde(rename = "@ResultClassID")]
  pub result_class_id: String,
  #[serde(rename = "@UserDriven")]
  pub user_driven: String,
  #[serde(rename = "@Distance")]
  pub distance: String,
  #[serde(rename = "@CallableShortName")]
  pub callable_short_name: String,
}

/// Группы операций.
// TODO
#[derive(Debug, Deserialize, Clone)]
pub struct MethodsGroups {}

/// Дочерние ТБП.
// TODO
#[derive(Debug, Deserialize, Clone)]
pub struct ChildClasses {}

//====================================================================================================================
// Представления и их строки
//====================================================================================================================

/// Данные представления.
#[derive(Debug, Deserialize, Clone)]
pub struct ViewData {
  #[serde(default, rename = "$value")]
  pub body: Vec<Row>,
}

/// Строка данных представления.
#[derive(Debug, Deserialize, Clone)]
pub struct Row {
  #[serde(default, rename = "$value")]
  pub body: Vec<RowItem>,
}

/// Колонка в строке данных.
#[derive(Debug, Deserialize, Clone)]
pub struct RowItem {
  #[serde(rename = "@ColumnName")]
  pub column_name: String,
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Список колонок представления.
#[derive(Debug, Deserialize, Clone)]
pub struct Columns {
  #[serde(default, rename = "$value")]
  pub body: Vec<Column>,
}

/// Описание колонки представления.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Width")]
  pub width: u32,
  #[serde(rename = "@Align")]
  pub align: u8, // TODO: Left = 0, Center = 1, Right = 2
  #[serde(rename = "@Position")]
  pub position: u32,
  #[serde(rename = "@Qual")]
  pub qual: String,
  #[serde(rename = "@Alias")]
  pub alias: String,
  #[serde(rename = "@Base")]
  pub base: ColumnBase,
  #[serde(rename = "@IsEditable", with = "option_bool_as_str")]
  pub is_editable: Option<bool>,
  #[serde(rename = "@IsSizeable", with = "bool_as_str")]
  pub is_sizeable: bool,
  #[serde(rename = "@IsCellStyle", with = "bool_as_str")]
  pub is_cell_style: bool,
  #[serde(rename = "@IsInvisible")]
  pub is_invisible: u8, // TODO: Visible = 0, Hidden = 2
  #[serde(rename = "@TargetClassID", skip_serializing_if = "Option::is_none")]
  pub target_class_id: Option<String>,
  #[serde(
    rename = "@ReferenceType",
    with = "option_bool_as_str",
    skip_serializing_if = "Option::is_none"
  )]
  pub reference_type: Option<bool>,
  #[serde(rename = "@Logging", skip_serializing_if = "Option::is_none")]
  pub logging: Option<Logging>,
  #[serde(rename = "@AbilityPerformOperation", skip_serializing_if = "Option::is_none")]
  pub ability_perform_operation: Option<bool>,
  #[serde(rename = "@ReferenceID", skip_serializing_if = "Option::is_none")]
  pub reference_id: Option<String>,
}

/// Базовый тип данных колонки.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ColumnBase {
  String,
  Number,
  Date,
  Reference,
  Collection,
}

/// Настройка логирования для колонки.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Logging {
  #[serde(rename = "0")]
  None,
  #[serde(rename = "D")]
  Delete,
}

/// Список представлений ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct Views {
  #[serde(default, rename = "$value")]
  pub body: Vec<View>,
}

/// Описание представления.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
  #[serde(rename = "@ID")]
  pub id: u64,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@IsDefault", with = "bool_as_str")]
  pub is_default: bool,
  #[serde(rename = "@CellStyleScript")]
  pub cell_style_script: Option<String>,
  #[serde(rename = "@Properties")]
  pub properties: String,
  #[serde(rename = "@Distance")]
  pub distance: u32,
  #[serde(rename = "@SourceID", skip_serializing_if = "Option::is_none")]
  pub source_id: Option<i64>,
  #[serde(rename = "@FilterMethodShortName")]
  pub filter_method_short_name: Option<String>,
  #[serde(rename = "@FilterMethodProperties")]
  pub filter_method_properties: Option<String>,
  #[serde(rename = "@ExtensionID", skip_serializing_if = "Option::is_none")]
  pub extension_id: Option<i64>,
  #[serde(rename = "@ObjectRights")]
  pub object_rights: u32,
  #[serde(rename = "@ToPrinter", with = "bool_as_str")]
  pub to_printer: bool,
  #[serde(rename = "@ToFile", with = "bool_as_str")]
  pub to_file: bool,
  #[serde(rename = "@Hints")]
  pub hints: Option<String>,
  #[serde(rename = "@OrderBy")]
  pub order_by: Option<String>,
}

//====================================================================================================================
// Навигация, справочники и меню
//====================================================================================================================

/// Пользовательское меню.
// TODO
#[derive(Debug, Deserialize, Clone)]
pub struct UserMenu {}

/// Список справочников.
#[derive(Debug, Deserialize, Clone)]
pub struct Guides {
  #[serde(default, rename = "$value")]
  pub body: Vec<GuideClass>,
}

/// Описание справочника.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "Class")]
pub struct GuideClass {
  #[serde(rename = "@GroupID")]
  pub group_id: Option<String>,
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@BaseClassID")]
  pub base_class_id: String,
  #[serde(rename = "@EntityID")]
  pub entity_id: String,
  #[serde(rename = "@IsKernelType")]
  pub is_kernel_type: String,
  #[serde(rename = "@ClassInterface")]
  pub class_interface: String,
  #[serde(rename = "@Flags")]
  pub flags: Flags,
}

/// Список групп справочников.
#[derive(Debug, Deserialize, Clone)]
pub struct GuidesGroups {
  #[serde(default, rename = "$value")]
  pub body: Vec<GuidesGroup>,
}

/// Описание группы справочников.
#[derive(Debug, Deserialize, Clone)]
pub struct GuidesGroup {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
}

/// Список всех ТПБ (не справочников) системы.
#[derive(Debug, Deserialize, Clone)]
pub struct Types {
  #[serde(default, rename = "$value")]
  pub body: Vec<Class>,
}

/// Описание ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct Class {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@BaseClassID")]
  pub base_class_id: String,
  #[serde(rename = "@EntityID")]
  pub entity_id: String,
  #[serde(rename = "@MenuCaption")]
  pub menu_caption: String,
  #[serde(rename = "@IsKernelType", with = "bool_as_str")]
  pub is_kernel_type: bool,
  #[serde(rename = "@ClassInterface")]
  pub class_interface: String,
  #[serde(rename = "@IsAccessible", with = "bool_as_str")]
  pub is_accessible: bool,
  #[serde(rename = "@Flags")]
  pub flags: Flags,
  #[serde(rename = "@PadLength")]
  pub pad_length: Option<u32>,
  #[serde(rename = "@DataSize")]
  pub data_size: Option<u32>,
  #[serde(rename = "@DataPrecision")]
  pub data_precision: Option<u32>,
  #[serde(rename = "@Properties")]
  pub properties: Option<String>,
}
