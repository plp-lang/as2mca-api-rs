//! Модуль, описывающий XML‑ответы сервера.
//!
//! Ответ всегда обёрнут в контейнер [`Response`], внутри которого находится перечисление [`ResponseBody`].
//! Каждый вариант перечисления соответствует конкретному типу ответа (сессия, список операций, данные представления и т.п.).
//!
//! # Пример
//! ```no_run
//! use as2mca_api::responses::{Response, ResponseBody};
//! # let xml = r#"<Response><Session ID="sess" DebugPipeName="pipe"/></Response>"#;
//!
//! let parsed: Response = quick_xml::de::from_str(xml).unwrap();
//! if let ResponseBody::Session(session) = parsed.body {
//!     println!("Session ID: {}", session.session_id);
//! }
//! ```

use serde::{Deserialize, Serialize};

use crate::serde_helpers::{empty_string_as_none, string_as_bool, string_as_option_bool};

/// Базовая обертка XML-ответа от сервера.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Response")]
pub struct Response {
  #[serde(rename = "$value")]
  pub body: ResponseBody,
}

/// Тело ответа
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseBody {
  ClientScript(ClientScript),
  Result(MethodResult),
  Validate(Validate),
  LockResult(LockResult),
  Class(Class),
  NotFound(NotFound),
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
  ChildClasses(ChildClasses),
  Views(Views),
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
  SystemContextInfo(SystemContextInfo),
  CoreInfo(CoreInfo),
  Settings(Settings),
  SystemInfo(SystemInfo),
  Limit(Limit),
  Attribute(Attribute),
  Application(Application),
  HelpSystemInfo(HelpSystemInfo),
  StreamData(StreamData),
}

/// Ответ "Not Found" (пустой).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotFound {}

/// Пустой ответ, подтверждающий успешное выполнение действия (например, отключение сессии).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Done {}

/// Структура ошибки API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Error {
  #[serde(rename = "@Text")]
  pub text: String,
  #[serde(rename = "ServerErrorInfo")]
  pub body: ServerErrorInfo,
}

/// Детали серверной ошибки.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerErrorInfo {
  #[serde(rename = "@Text")]
  pub text: String,
}

//======================================================================================================================
// Сессия и авторизация
//======================================================================================================================

/// Ответ с данными успешно инициализированной сессии.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
  #[serde(rename = "@ID")]
  pub session_id: String,
  #[serde(rename = "@DebugPipeName")]
  pub debug_pipe_name: String,
}

/// Ответ с URL для авторизации.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticationURL {
  #[serde(rename = "@URL")]
  pub url: String,
}

//======================================================================================================================
// Информация о системе
//======================================================================================================================

/// Информация о версии протокола.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolInfo {
  #[serde(rename = "@Version")]
  pub version: String,
}

/// Информация о версии сервера.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerInfo {
  #[serde(rename = "@Version")]
  pub version: String,
}

/// Информация о системе.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// Информация о системе.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemContextInfo {
  /// Системная дата
  #[serde(rename = "@SystemDate")]
  pub system_date: String,
  /// Системное имя
  #[serde(rename = "@SystemName")]
  pub system_name: String,
  /// Дополнительная информация
  #[serde(rename = "@SystemInfo")]
  pub system_info: String,
}

/// Список системных настроек.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
  #[serde(default, rename = "$value")]
  pub body: Vec<Setting>,
}

/// Конкретная системная настройка (ключ-значение).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Setting {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(default, rename = "@Value", skip_serializing_if = "Option::is_none")]
  pub value: Option<String>,
}

/// Результат проверки доступности NOVO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NovoAllowedCheckResult {
  #[serde(rename = "@Value", with = "string_as_bool")]
  pub value: bool,
}

/// Информация о включенности системной опции.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionInfo {
  #[serde(rename = "@Enabled", with = "string_as_bool")]
  pub enabled: bool,
}

/// Значение системного параметра.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemInfo {
  #[serde(
    default,
    rename = "@Value",
    skip_serializing_if = "Option::is_none",
    deserialize_with = "empty_string_as_none::deserialize"
  )]
  pub value: Option<String>,
}

/// Значение системного ограничения (лимита).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Limit {
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Значение атрибута системного контекста.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attribute {
  #[serde(
    default,
    rename = "@Value",
    skip_serializing_if = "Option::is_none",
    deserialize_with = "empty_string_as_none::deserialize"
  )]
  pub value: Option<String>,
}

/// Имя приложения
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Application {
  #[serde(rename = "@Name")]
  pub name: String,
}

/// Количество элементов в справочной системе.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpSystemInfo {
  #[serde(rename = "@ItemsCount")]
  pub items_count: u64,
}

/// URL-адрес ресурса WebView-модуля.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamData {
  #[serde(rename = "@URL")]
  pub url: String,
}

//======================================================================================================================
// Информация о пользователе
//======================================================================================================================

/// Базовая информация о пользователе.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
}

/// Информация о привилегиях пользователя.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserPrivileged {
  #[serde(rename = "@IsPrivileged", with = "string_as_bool")]
  pub is_privileged: bool,
}

/// Содержимое элемента `<User>` — зависит от метода API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserContent {
  /// Ответ `UserInfoGet`: `<User Name="..." ShortName="..." Properties="..."/>`
  Info(User),
  /// Ответ `SystemUserPrivilegedGet`: `<User IsPrivileged="..."/>`
  Privileged(UserPrivileged),
}

/// Значение свойства профиля пользователя.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserProfileProperty {
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Универсальный результат проверки (например, вхождения в группу).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResult {
  #[serde(rename = "@Value")]
  pub value: String,
}

//======================================================================================================================
// Отладка
//======================================================================================================================

/// Текст из отладочного канала.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipeText {
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Отладочный текст.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugText {
  #[serde(rename = "@Value")]
  pub value: String,
}

//======================================================================================================================
// ТБП и их экземпляры
//======================================================================================================================

/// Идентификатор ТБП и ключ архива экземпляра.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectClassAndArchiveKey {
  #[serde(rename = "@ClassID", default, skip_serializing_if = "Option::is_none")]
  pub class_id: Option<String>,
  #[serde(rename = "@ArchiveKey", default, skip_serializing_if = "Option::is_none")]
  pub archive_key: Option<String>,
}

/// Обратная ссылка на экземпляр.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// Список обратных ссылок.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackwardReferences {
  #[serde(default, rename = "$value")]
  pub body: Vec<BackwardReference>,
}

/// Переход состояний ТБП.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@MethodShortName", deserialize_with = "empty_string_as_none::deserialize")]
  pub method_short_name: Option<String>,
  #[serde(rename = "@InitialStateID")]
  pub initial_state_id: String,
  #[serde(rename = "@FinalStateID")]
  pub final_state_id: String,
}

/// Список переходов.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transitions {
  #[serde(default, rename = "$value")]
  pub transitions: Vec<Transition>,
}

/// Состояние ТБП.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@IndexUse")]
  pub index_use: i64,
}

/// Список состояний.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct States {
  #[serde(default, rename = "$value")]
  pub states: Vec<State>,
}

/// Дочерний ТБП.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildClass {
  #[serde(rename = "@ID")]
  pub id: String,
}

/// Список дочерних ТБП.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildClasses {
  #[serde(default, rename = "$value")]
  pub child_classes: Vec<ChildClass>,
}

//======================================================================================================================
// Операции
//======================================================================================================================

/// Структура операции.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

  #[serde(rename = "@ScriptID", default, skip_serializing_if = "Option::is_none")]
  pub script_id: Option<String>,

  /// Короткое имя возвращаемого типа операцией.
  #[serde(rename = "@ResultClassID", default, skip_serializing_if = "Option::is_none")]
  pub result_class_id: Option<String>,

  #[serde(rename = "@UserDriven", default, skip_serializing_if = "Option::is_none")]
  pub user_driven: Option<u8>,
  #[serde(rename = "@FormID", default, skip_serializing_if = "Option::is_none")]
  pub form_id: Option<i64>,
  #[serde(rename = "@ReportType", default, skip_serializing_if = "Option::is_none")]
  pub report_type: Option<String>,
  #[serde(rename = "@ReportTemplate", default, skip_serializing_if = "Option::is_none")]
  pub report_template: Option<String>,
}

/// Тип операции.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
  /// `O` — выбор.
  #[serde(rename = "O")]
  Choice,
  /// `P` — печать.
  #[serde(rename = "P")]
  Print,
}

/// Список операций.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Methods {
  #[serde(default, rename = "$value")]
  pub body: Vec<Method>,
}

/// Описание входного параметра операции.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

  #[serde(rename = "@ViewID", default, skip_serializing_if = "Option::is_none")]
  pub view_id: Option<i64>,
  #[serde(rename = "@ViewClassID", default, skip_serializing_if = "Option::is_none")]
  pub view_class_id: Option<String>,
  #[serde(rename = "@ViewFilter", default, skip_serializing_if = "Option::is_none")]
  pub view_filter: Option<String>,

  /// Значение по умолчанию.
  #[serde(rename = "@DefaultValue", default, skip_serializing_if = "Option::is_none")]
  pub default_value: Option<String>,
}

/// Тип ссылки.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReferenceType {
  /// default
  #[serde(rename = "D")]
  Default,
  /// `table of`
  #[serde(rename = "T")]
  TableOf,
  /// `ref`
  #[serde(rename = "R")]
  Ref,
}

/// Направление параметра.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
  /// `default`
  #[serde(rename = "D")]
  Default,
  /// `in`
  #[serde(rename = "I")]
  In,
  /// `in out`
  #[serde(rename = "B")]
  InOut,
  /// `out`
  #[serde(rename = "O")]
  Out,
}

/// Список входных параметров.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodParameters {
  #[serde(default, rename = "$value")]
  pub parameters: Vec<MethodParameter>,
}

/// Описание публичной переменной операции.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodVariable {
  /// Имя переменной.
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  /// ТБП переменной.
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@Position")]
  pub position: u32,
  #[serde(rename = "@ReferenceType")]
  pub reference_type: ReferenceType,
}

/// Список публичных переменных.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodVariables {
  #[serde(default, rename = "$value")]
  pub variables: Vec<MethodVariable>,
}

/// Описание элемента формы.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Control {
  ///  Идентификатор элемента.
  #[serde(rename = "@ID")]
  pub id: i64,

  /// Идентификатор операции.
  #[serde(rename = "@MethodID")]
  pub method_id: i64,

  /// Квалификатор
  #[serde(rename = "@Qualifier")]
  pub qualifier: String,

  /// Тип элемента формы. Наример тестовое поле или кнопка.
  #[serde(rename = "@Control")]
  pub control: ControlType,

  /// Заголовок.
  #[serde(rename = "@Caption")]
  pub caption: String,

  /// Кол-во пикселей отступа от верхнего края формы.
  #[serde(rename = "@Top")]
  pub top: i32,

  /// Кол-во пикселей отступа от левого края формы.
  #[serde(rename = "@Left")]
  pub left: i32,

  /// Высота элемента в пикселях.
  #[serde(rename = "@Height")]
  pub height: u32,

  /// Ширины элемента в пикселях.
  #[serde(rename = "@Width")]
  pub width: u32,

  /// Индекс табуляции.
  #[serde(rename = "@TabIndex")]
  pub tab_index: i32,

  /// Позиция.
  #[serde(rename = "@Position")]
  pub position: u32,

  /// Имя для валидации.
  #[serde(rename = "@ValidateName")]
  pub validate_name: String,

  /// Идентификатор родительского элемента.
  /// Это число, но иногда приходит как `ParentID=""`.
  #[serde(rename = "@ParentID")]
  pub parent_id: String,

  /// ТБП значения.
  #[serde(rename = "@ClassID", default, skip_serializing_if = "Option::is_none")]
  pub class_id: Option<String>,

  /// Зависимость.
  #[serde(rename = "@Depend", default, skip_serializing_if = "Option::is_none")]
  pub depend: Option<i64>,

  /// Свойства
  #[serde(rename = "@Properties", default, skip_serializing_if = "Option::is_none")]
  pub properties: Option<String>,

  /// Подсказка.
  #[serde(rename = "@Tips", default, skip_serializing_if = "Option::is_none")]
  pub tips: Option<String>,
}

/// Тип элемента формы
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ControlType {
  Form,
  Label,
  Text,
  Object,
  Check,
  Button,
  Subform,
  Line,
  Memo,
  Frame,
  Date,
  Variant,
  Array,
  Panel,
  Combo,
  Number,
  Depend,
  Tabbed,
  Grid,
  Gridcol,
  Table,
}

/// Спиcок элементов на форме.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Controls {
  #[serde(default, rename = "$value")]
  pub controls: Vec<Control>,
}

/// Результат выполнения блока `Validate`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Validate {
  #[serde(default, rename = "@DebugText", skip_serializing_if = "Option::is_none")]
  pub debug_text: Option<String>,
  #[serde(rename = "@ObjectID", default, skip_serializing_if = "Option::is_none")]
  pub object_id: Option<i64>,
  #[serde(rename = "ControlsStates", skip_serializing_if = "Option::is_none")]
  pub controls_states: Option<ControlsStates>,
}

/// Состояние элемента на форме.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlState {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Результат выполнения блока `Execute`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Result")]
pub struct MethodResult {
  #[serde(default, rename = "@Value", skip_serializing_if = "Option::is_none")]
  pub value: Option<String>,
  #[serde(rename = "ControlsStates", skip_serializing_if = "Option::is_none")]
  pub controls_states: Option<ControlsStates>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlsStates {
  #[serde(rename = "ControlState", default, skip_serializing_if = "Vec::is_empty")]
  pub items: Vec<ControlState>,
}

/// Клиент-скрипт
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientScript {
  #[serde(rename = "@Text")]
  pub text: String,
}

// Информация об открытой форме.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MethodFrame {
  #[serde(rename = "@FrameID", default, skip_serializing_if = "Option::is_none")]
  pub frame_id: Option<i64>,
}

//======================================================================================================================
// Представления и данные
//======================================================================================================================

/// Данные представления.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewData {
  #[serde(default, rename = "$value")]
  pub row: Vec<Row>,
}

/// Строка данных представления.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Row {
  #[serde(default, rename = "$value")]
  pub row_item: Vec<RowItem>,
}

/// Значение колонки в строке.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowItem {
  #[serde(rename = "@ColumnName")]
  pub column_name: String,
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Описание колонки представления.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
  #[serde(rename = "@IsSizeable", with = "string_as_bool")]
  pub is_sizeable: bool,
  #[serde(rename = "@IsInvisible")]
  pub is_invisible: Invisible,
  #[serde(rename = "@AbilityPerformOperation")]
  pub ability_perform_operation: bool,
  #[serde(
    rename = "@IsCellStyle",
    default,
    with = "string_as_option_bool",
    skip_serializing_if = "Option::is_none"
  )]
  pub is_cell_style: Option<bool>,
  #[serde(
    rename = "@IsEditable",
    default,
    with = "string_as_option_bool",
    skip_serializing_if = "Option::is_none"
  )]
  pub is_editable: Option<bool>,
  #[serde(rename = "@ReferenceID", default, skip_serializing_if = "Option::is_none")]
  pub reference_id: Option<String>,
  #[serde(rename = "@TargetClassID", default, skip_serializing_if = "Option::is_none")]
  pub target_class_id: Option<String>,
  #[serde(rename = "@ReferenceType", default, skip_serializing_if = "Option::is_none")]
  pub reference_type: Option<u8>,
  #[serde(rename = "@Logging", default, skip_serializing_if = "Option::is_none")]
  pub logging: Option<Logging>,
}

/// Базовый тип данных колонки.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ColumnBase {
  Memo,
  Date,
  String,
  Number,
  Boolean,
  Reference,
  Collection,
  Ole,
  Null,
  State,
}

/// Выравнивание.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Align {
  #[serde(rename = "0")]
  Left = 0,
  #[serde(rename = "1")]
  Center = 1,
  #[serde(rename = "2")]
  Right = 2,
}

/// Видимость.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Invisible {
  #[serde(rename = "0")]
  Visible = 0,
  #[serde(rename = "1")]
  Connection = 1,
  #[serde(rename = "2")]
  Invisible = 2,
}

/// Логирование.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Logging {
  #[serde(rename = "0")]
  No,
  #[serde(rename = "1")]
  Yes,
  #[serde(rename = "D")]
  D,
}

/// Список колонок.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Columns {
  #[serde(default, rename = "$value")]
  pub body: Vec<Column>,
}

/// Описание представления.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct View {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@IsDefault")]
  pub is_default: bool,
  #[serde(rename = "@Properties")]
  pub properties: String,
  #[serde(rename = "@Distance")]
  pub distance: u8,
  #[serde(rename = "@ObjectRights")]
  pub object_rights: u8,
  #[serde(rename = "@ToPrinter")]
  pub to_printer: bool,
  #[serde(rename = "@ToFile")]
  pub to_file: bool,

  #[serde(rename = "@OrderBy", default, skip_serializing_if = "Option::is_none")]
  pub order_by: Option<String>,
  #[serde(rename = "@Hints", default, skip_serializing_if = "Option::is_none")]
  pub hints: Option<String>,
  #[serde(rename = "@CellStyleScript", default, skip_serializing_if = "Option::is_none")]
  pub cell_style_script: Option<String>,
  #[serde(rename = "@SourceID", default, skip_serializing_if = "Option::is_none")]
  pub source_id: Option<i64>,
  #[serde(rename = "@ExtensionID", default, skip_serializing_if = "Option::is_none")]
  pub extension_id: Option<i64>,
  #[serde(rename = "@FilterMethodShortName", default, skip_serializing_if = "Option::is_none")]
  pub filter_method_short_name: Option<String>,
  #[serde(rename = "@FilterMethodProperties", default, skip_serializing_if = "Option::is_none")]
  pub filter_method_properties: Option<String>,
}

/// Список представлений.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Views {
  #[serde(default, rename = "$value")]
  pub body: Vec<View>,
}

//======================================================================================================================
// Навигация, справочники и меню
//======================================================================================================================

/// Список справочников.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Guides {
  #[serde(default, rename = "$value")]
  pub body: Vec<Class>,
}

/// Группа справочников.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidesGroup {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
}

/// Список групп справочников.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidesGroups {
  #[serde(default, rename = "$value")]
  pub body: Vec<GuidesGroup>,
}

/// Список cправочников.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Classes {
  #[serde(default, rename = "$value")]
  pub body: Vec<Class>,
}

/// Список ТПБ (не справочников).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Types {
  #[serde(default, rename = "$value")]
  pub body: Vec<Class>,
}

/// Описание ТБП (типа).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Class {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@BaseClassID")]
  pub base_class_id: String,
  #[serde(rename = "@EntityID")]
  pub entity_id: String,
  #[serde(rename = "@IsKernelType", with = "string_as_bool")]
  pub is_kernel_type: bool,
  #[serde(rename = "@ClassInterface")]
  pub class_interface: String,
  #[serde(rename = "@Flags")]
  pub flags: Flags,

  #[serde(rename = "@MenuCaption", default, skip_serializing_if = "Option::is_none")]
  pub menu_caption: Option<String>,
  #[serde(
    rename = "@IsAccessible",
    default,
    skip_serializing_if = "Option::is_none",
    with = "string_as_option_bool"
  )]
  pub is_accessible: Option<bool>,
  #[serde(rename = "@PadLength", default, skip_serializing_if = "Option::is_none")]
  pub pad_length: Option<u8>,
  #[serde(rename = "@DataSize", default, skip_serializing_if = "Option::is_none")]
  pub data_size: Option<u32>,
  #[serde(rename = "@DataPrecision", default, skip_serializing_if = "Option::is_none")]
  pub data_precision: Option<u8>,
  #[serde(rename = "@Properties", default, skip_serializing_if = "Option::is_none")]
  pub properties: Option<String>,
  #[serde(rename = "@GroupID", default, skip_serializing_if = "Option::is_none")]
  pub group_id: Option<String>,
}

/// Тип справочника.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BaseClassID {
  Structure,
}

//======================================================================================================================
// Блокировки
//======================================================================================================================

/// Результат блокировки экземпляра
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockResult {
  #[serde(rename = "@Message", default, skip_serializing_if = "Option::is_none")]
  pub message: Option<String>,
}

/// Флаги с тремя состояниями: 0 (выключен), 1 (включен), 2 (специальный/альтернативный)
/// Хранятся как массив из 25 значений.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Flags([u8; 25]);

impl Flags {
  pub const LEN: usize = 25;

  /// Создаёт флаги из массива значений
  #[must_use]
  pub const fn new(values: [u8; 25]) -> Self {
    Self(values)
  }

  /// Получает значение флага по индексу
  #[must_use]
  pub const fn get(&self, index: usize) -> u8 {
    self.0[index]
  }

  /// Проверяет, установлен ли флаг (значение != 0)
  #[must_use]
  pub const fn has_flag(&self, index: usize) -> bool {
    self.0[index] != 0
  }

  /// Проверяет, что флаг имеет конкретное значение
  #[must_use]
  pub const fn is(&self, index: usize, value: u8) -> bool {
    self.0[index] == value
  }
}

impl TryFrom<String> for Flags {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    if value.len() != Self::LEN {
      return Err(format!(
        "Invalid flags length: expected {}, got {}",
        Self::LEN,
        value.len()
      ));
    }

    let mut result = [0u8; 25];
    for (i, c) in value.chars().enumerate() {
      result[i] = match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        _ => {
          return Err(format!("Invalid character '{c}' at position {i} in flags '{value}'"));
        }
      };
    }

    Ok(Self(result))
  }
}

impl From<Flags> for String {
  #[allow(clippy::cast_lossless)]
  fn from(flags: Flags) -> Self {
    flags
      .0
      .iter()
      .map(|&b| char::from_digit(b as u32, 10).unwrap())
      .collect()
  }
}
