use crate::models::{
  requests::{DebugPipeName, SessionId},
  utils::{Flags, bool_as_bool},
};
use serde::{Deserialize, Deserializer};

/// Базовая обертка XML-ответа от сервера.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "Response")]
pub struct Response {
  #[serde(rename = "$value")]
  pub body: ResponseBody,
}

/// Тело ответа
#[derive(Debug, Deserialize, Clone)]
pub enum ResponseBody {
  Validate(Validate),
  LockResult(LockResult),
  Classes(Classes),
  MethodVariables(MethodVariables),
  Controls(Controls),
  MethodParameters(MethodParameters),
  MethodFrame(MethodFrame),
  ObjectClassAndArchiveKey(ObjectClassAndArchiveKey),
  DebugText(DebugText),
  Setting(Setting),
  PipeText(PipeText),
  BackwardReferences(BackwardReferences),
  ViewData(ViewData),
  Transitions(Transitions),
  States(States),
  Columns(Columns),
  Methods(Methods),
  MethodsGroups(MethodsGroups),
  ChildClasses(ChildClasses),
  Views(Views),
  UserMenu(UserMenu),
  Guides(Guides),
  GuidesGroups(GuidesGroups),
  Types(Types),
  CheckResult(CheckResult),
  OptionInfo(OptionInfo),
  User(UserContent),
  UserProfileProperty(UserProfileProperty),
  NovoAllowedCheckResult(NovoAllowedCheckResult),
  AuthenticationURL(AuthenticationURL),
  ProtocolInfo(ProtocolInfo),
  Session(Session),
  Done(Done),
  Error(Error),
  ServerInfo(ServerInfo),
  CoreInfo(CoreInfo),
  Settings(Settings),
}

/// Содержимое элемента `<User>` — зависит от метода API.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum UserContent {
  /// Ответ `UserInfoGet`: `<User Name="..." ShortName="..." Properties="..."/>`
  Info(User),
  /// Ответ `SystemUserPrivilegedGet`: `<User IsPrivileged="..."/>`
  Privileged(UserPrivileged),
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
pub struct UserPrivileged {
  #[serde(rename = "@IsPrivileged", with = "bool_as_bool")]
  pub is_privileged: bool,
}

/// Значение свойства профиля пользователя.
#[derive(Debug, Deserialize, Clone)]
pub struct UserProfileProperty {
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Универсальный результат проверки (например, вхождения в группу).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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

/// Дочерние ТБП.
// TODO
#[derive(Debug, Deserialize, Clone)]
pub struct ChildClasses {}

//======================================================================================================================
// Операции
//======================================================================================================================

/// Список операций ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct Methods {
  #[serde(default, rename = "$value")]
  pub body: Vec<Method>,
}

/// Структура операции.
#[derive(Debug, Deserialize, Clone)]
pub struct Method {
  /// ID операции.
  #[serde(rename = "@ID")]
  pub id: i64,

  /// Полное наименование.
  #[serde(rename = "@Name")]
  pub name: String,

  /// Короткое имя.
  #[serde(rename = "@ShortName")]
  pub short_name: String,

  /// Тип операции.
  #[serde(rename = "@Type")]
  pub r#type: MethodType,

  #[serde(rename = "@FormClassID")]
  pub form_class_id: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
  #[serde(rename = "@Distance")]
  pub distance: u8,
  #[serde(rename = "@CallableShortName")]
  pub callable_short_name: String,

  #[serde(rename = "@ScriptID", default)]
  pub script_id: Option<String>,

  /// Короткое имя возвращаемого типа операцией.
  #[serde(rename = "@ResultClassID", default)]
  pub result_class_id: Option<String>,

  #[serde(rename = "@UserDriven", default)]
  pub user_driven: Option<u8>,
  #[serde(rename = "@FormID", default)]
  pub form_id: Option<i64>,
  #[serde(rename = "@ReportType", default)]
  pub report_type: Option<String>,
  #[serde(rename = "@ReportTemplate", default)]
  pub report_template: Option<String>,
}

/// Тип операции.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum MethodType {
  /// `C` — конструктор.
  #[serde(rename = "C")]
  Constructor,
  /// `G` — списочная операция.
  #[serde(rename = "G")]
  Batch,
  /// `M` — простая операция.
  #[serde(rename = "M")]
  Method,
  /// `R` — отчёт.
  #[serde(rename = "R")]
  Report,
  /// `S` — групповая операция.
  #[serde(rename = "S")]
  Group,
  /// `Y` — деструктор.
  #[serde(rename = "Y")]
  Destructor,
}

/// Список входных параметров операции.
#[derive(Debug, Deserialize, Clone)]
pub struct MethodParameters {
  #[serde(default, rename = "$value")]
  pub parameters: Vec<MethodParameter>,
}

/// Описание входного параметра операции.
#[derive(Debug, Deserialize, Clone)]
pub struct MethodParameter {
  /// Короткое имя параметра.
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@Position")]
  pub position: u32,
  #[serde(rename = "@ReferenceType")]
  pub reference_type: ReferenceType,
  #[serde(rename = "@Direction")]
  pub direction: Direction,

  /// Значение по умолчанию.
  #[serde(rename = "@DefaultValue", default)]
  pub default_value: Option<String>,
}

/// Список публичных переменных операции.
#[derive(Debug, Deserialize, Clone)]
pub struct MethodVariables {
  #[serde(default, rename = "$value")]
  pub variables: Vec<MethodVariable>,
}

/// Описание публичной переменной операции.
#[derive(Debug, Deserialize, Clone)]
pub struct MethodVariable {
  /// Имя переменной.
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  /// Тип переменной.
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@Position")]
  pub position: u32,
  #[serde(rename = "@ReferenceType")]
  pub reference_type: ReferenceType,
}

/// Тип ссылочного типа
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceType {
  D,
  /// `table of`?
  T,
}

/// TODO
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  D,
  I,
}

/// Спиcок элементов на форме.
#[derive(Debug, Deserialize, Clone)]
pub struct Controls {
  #[serde(default, rename = "$value")]
  pub controls: Vec<Control>,
}

/// Структура элемента на форме
#[derive(Debug, Deserialize, Clone)]
pub struct Control {
  /// ID элемента.
  #[serde(rename = "@ID")]
  pub id: i64,

  /// ID операции, элемент формы которой предналежит.
  #[serde(rename = "@MethodID")]
  pub method_id: i64,

  #[serde(rename = "@Qualifier")]
  pub qualifier: String,

  /// Тип элемента формы. Наример тестовое поле или кнопка.
  #[serde(rename = "@Control")]
  pub control: ControlType,

  #[serde(rename = "@Caption")]
  pub caption: String,

  /// Кол-во пикселей отступа от верхнего края формы.
  #[serde(rename = "@Top")]
  pub top: u32,

  /// Кол-во пикселей отступа от левого края формы.
  #[serde(rename = "@Left")]
  pub left: u32,

  /// Высота элемента в пикселях.
  #[serde(rename = "@Height")]
  pub height: u32,

  /// Ширины элемента в пикселях.
  #[serde(rename = "@Width")]
  pub width: u32,

  #[serde(rename = "@TabIndex")]
  pub tab_index: u32,
  #[serde(rename = "@Position")]
  pub position: u32,

  /// Имя элемента по которому к нему можно обратится из кода.
  #[serde(rename = "@ValidateName")]
  pub validate_name: String,

  /// ID родительского элемента на форме.
  /// Это число, но иногда приходит как `ParentID=""`, считаем что родитель отсутствует.
  #[serde(rename = "@ParentID", default, deserialize_with = "deserialize_optional_number")]
  pub parent_id: Option<i64>,

  /// Короткое имя ТБП (тип, справочник) которому соответствует значение в элементе.
  #[serde(rename = "@ClassID", default)]
  pub class_id: Option<String>,
  #[serde(rename = "@Depend", default)]
  pub depend: Option<i64>,
  #[serde(rename = "@Properties", default)]
  pub properties: Option<String>,

  /// Тект, который всплывает при наведении на элемент курсором.
  #[serde(rename = "@Tips", default)]
  pub tips: Option<String>,
}

/// Тип элемента формы
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ControlType {
  Form,
  Label,
  Text,
  Object,
  Check,
  Button,
}

/// Результат выполнения блока `Validate` операции.
#[derive(Debug, Deserialize, Clone)]
pub struct Validate {
  #[serde(rename = "@DebugText")]
  pub debug_text: String,
  #[serde(rename = "$value")]
  pub controls_states: ControlsStates,
}

/// Список значений элементов формы операции.
#[derive(Debug, Deserialize, Clone)]
pub struct ControlsStates {
  #[serde(default, rename = "$value")]
  pub controls_states: Vec<ControlsState>,
}

/// Значение элемента формы операции.
#[derive(Debug, Deserialize, Clone)]
pub struct ControlsState {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Value")]
  pub value: String,
}

// TODO
#[derive(Debug, Deserialize, Clone)]
pub struct MethodFrame {
  #[serde(rename = "@FrameID")]
  pub frame_id: i64,
}

// TODO
#[derive(Debug, Deserialize, Clone)]
pub struct MethodsGroups {}

//====================================================================================================================
// Представления и их строки
//====================================================================================================================

/// Данные представления.
#[derive(Debug, Deserialize, Clone)]
pub struct ViewData {
  #[serde(default, rename = "$value")]
  pub row: Vec<Row>,
}

/// Строка данных представления.
#[derive(Debug, Deserialize, Clone)]
pub struct Row {
  #[serde(default, rename = "$value")]
  pub row_item: Vec<RowItem>,
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
#[derive(Debug, Clone, Deserialize)]
pub struct Column {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Width")]
  pub width: u32,
  #[serde(rename = "@Align")]
  pub align: Align,
  #[serde(rename = "@Position")]
  pub position: u32,
  #[serde(rename = "@Qual")]
  pub qual: String,
  #[serde(rename = "@Alias")]
  pub alias: String,
  #[serde(rename = "@Base")]
  pub base: ColumnBase,
  #[serde(rename = "@IsSizeable")]
  pub is_sizeable: u8,
  #[serde(rename = "@IsCellStyle")]
  pub is_cell_style: u8,
  #[serde(rename = "@IsInvisible")]
  pub is_invisible: Invisible,
  #[serde(rename = "@AbilityPerformOperation")]
  pub ability_perform_operation: bool,

  #[serde(rename = "@IsEditable", default)]
  pub is_editable: Option<u8>,
  #[serde(rename = "@ReferenceID", default)]
  pub reference_id: Option<String>,
  #[serde(rename = "@TargetClassID", default)]
  pub target_class_id: Option<String>,
  #[serde(rename = "@ReferenceType", default)]
  pub reference_type: Option<u8>,
  #[serde(rename = "@Logging", default)]
  pub logging: Option<Logging>,
}

#[derive(Debug, Clone, Deserialize)]
#[repr(u8)]
pub enum Align {
  #[serde(rename = "0")]
  Left = 0,
  #[serde(rename = "1")]
  Center = 1,
  #[serde(rename = "2")]
  Right = 2,
}

#[derive(Debug, Clone, Deserialize)]
#[repr(u8)]
pub enum Invisible {
  #[serde(rename = "0")]
  Visible = 0,
  #[serde(rename = "2")]
  Hidden = 2,
}

/// Базовый тип данных колонки.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ColumnBase {
  String,
  Number,
  Date,
  Reference,
  Collection,
}

/// Настройка логирования для колонки.
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
pub struct View {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@IsDefault")]
  pub is_default: u8,
  #[serde(rename = "@Properties")]
  pub properties: String,
  #[serde(rename = "@Distance")]
  pub distance: u8,
  #[serde(rename = "@ObjectRights")]
  pub object_rights: u8,
  #[serde(rename = "@ToPrinter")]
  pub to_printer: u8,
  #[serde(rename = "@ToFile")]
  pub to_file: u8,

  #[serde(rename = "@OrderBy", default)]
  pub order_by: Option<String>,
  #[serde(rename = "@Hints", default)]
  pub hints: Option<String>,
  #[serde(rename = "@CellStyleScript", default)]
  pub cell_style_script: Option<String>,
  #[serde(rename = "@SourceID", default)]
  pub source_id: Option<i64>,
  #[serde(rename = "@ExtensionID", default)]
  pub extension_id: Option<i64>,
  #[serde(rename = "@FilterMethodShortName", default)]
  pub filter_method_short_name: Option<String>,
  #[serde(rename = "@FilterMethodProperties", default)]
  pub filter_method_properties: Option<String>,
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
  pub body: Vec<Class>,
}

/// Тип справочника.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BaseClassID {
  Structure,
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

/// Список типов/ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct Classes {
  #[serde(default, rename = "$value")]
  pub body: Vec<Class>,
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
  #[serde(rename = "@IsKernelType")]
  pub is_kernel_type: u8,
  #[serde(rename = "@ClassInterface")]
  pub class_interface: String,
  #[serde(rename = "@Flags")]
  pub flags: Flags,

  #[serde(rename = "@MenuCaption", default)]
  pub menu_caption: Option<String>,
  #[serde(rename = "@IsAccessible", default)]
  pub is_accessible: Option<u8>,
  #[serde(rename = "@PadLength", default)]
  pub pad_length: Option<u8>,
  #[serde(rename = "@DataSize", default)]
  pub data_size: Option<u32>,
  #[serde(rename = "@DataPrecision", default)]
  pub data_precision: Option<u8>,
  #[serde(rename = "@Properties", default)]
  pub properties: Option<String>,
  #[serde(rename = "@GroupID", default)]
  pub group_id: Option<String>,
}

//====================================================================================================================
// Блокировки
//====================================================================================================================

/// Результат блокировки экземпляра
#[derive(Debug, Deserialize, Clone)]
pub struct LockResult {
  #[serde(rename = "@Message", default)]
  pub message: Option<String>,
}

fn deserialize_optional_number<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
  D: Deserializer<'de>,
  T: std::str::FromStr,
  T::Err: std::fmt::Display,
{
  let s: Option<String> = Option::deserialize(deserializer)?;
  match s {
    None => Ok(None),
    Some(s) if s.is_empty() => Ok(None),
    Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
  }
}
