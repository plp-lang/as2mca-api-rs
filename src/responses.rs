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

use serde::Deserialize;

use crate::serde_helpers::{empty_string_as_none, string_as_bool, unwrap_list};

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

/// Ответ "Not Found" (пустой).
#[derive(Debug, Deserialize, Clone)]
pub struct NotFound {}

/// Пустой ответ, подтверждающий успешное выполнение действия (например, отключение сессии).
#[derive(Debug, Deserialize, Clone)]
pub struct Done {}

/// Структура ошибки API.
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
  pub session_id: String,
  #[serde(rename = "@DebugPipeName")]
  pub debug_pipe_name: String,
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

/// Информация о системе.
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
  #[serde(rename = "$value", default)]
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
  #[serde(rename = "@Value", with = "string_as_bool")]
  pub value: bool,
}

/// Информация о включенности системной опции.
#[derive(Debug, Deserialize, Clone)]
pub struct OptionInfo {
  #[serde(rename = "@Enabled", with = "string_as_bool")]
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
  #[serde(rename = "@IsPrivileged", with = "string_as_bool")]
  pub is_privileged: bool,
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

/// Значение свойства профиля пользователя.
#[derive(Debug, Deserialize, Clone)]
pub struct UserProfileProperty {
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Универсальный результат проверки (например, вхождения в группу).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CheckResult {
  #[serde(rename = "@Value", with = "string_as_bool")]
  pub value: bool,
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

/// Обратная ссылка на экземпляр.
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

/// Список обратных ссылок.
#[derive(Debug, Deserialize, Clone)]
pub struct BackwardReferences {
  #[serde(rename = "$value", default)]
  pub body: Vec<BackwardReference>,
}

/// Переход состояний ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct Transition {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@MethodShortName", with = "empty_string_as_none")]
  pub method_short_name: Option<String>,
  #[serde(rename = "@InitialStateID")]
  pub initial_state_id: String,
  #[serde(rename = "@FinalStateID")]
  pub final_state_id: String,
}

/// Список переходов.
#[derive(Debug, Deserialize, Clone)]
pub struct Transitions {
  #[serde(rename = "$value", default)]
  pub transitions: Vec<Transition>,
}

/// Состояние ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct State {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@IndexUse")]
  pub index_use: i64,
}

/// Список состояний.
#[derive(Debug, Deserialize, Clone)]
pub struct States {
  #[serde(rename = "$value", default)]
  pub states: Vec<State>,
}

/// Дочерний ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct ChildClass {
  #[serde(rename = "@ID")]
  pub id: String,
}

/// Список дочерних ТБП.
#[derive(Debug, Deserialize, Clone)]
pub struct ChildClasses {
  #[serde(rename = "$value", default)]
  pub child_classes: Vec<ChildClass>,
}

//======================================================================================================================
// Операции
//======================================================================================================================

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

/// Список операций.
#[derive(Debug, Deserialize, Clone)]
pub struct Methods {
  #[serde(rename = "$value", default)]
  pub body: Vec<Method>,
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

  #[serde(rename = "@ViewID", default)]
  pub view_id: Option<i64>,
  #[serde(rename = "@ViewClassID", default)]
  pub view_class_id: Option<String>,
  #[serde(rename = "@ViewFilter", default)]
  pub view_filter: Option<String>,

  /// Значение по умолчанию.
  #[serde(rename = "@DefaultValue", default)]
  pub default_value: Option<String>,
}

/// Тип ссылки.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceType {
  D,
  /// `table of`?
  T,
}

/// Направление параметра.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  D,
  I,
}

/// Список входных параметров.
#[derive(Debug, Deserialize, Clone)]
pub struct MethodParameters {
  #[serde(rename = "$value", default)]
  pub parameters: Vec<MethodParameter>,
}

/// Описание публичной переменной операции.
#[derive(Debug, Deserialize, Clone)]
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
#[derive(Debug, Deserialize, Clone)]
pub struct MethodVariables {
  #[serde(rename = "$value", default)]
  pub variables: Vec<MethodVariable>,
}

/// Описание элемента формы.
#[derive(Debug, Deserialize, Clone)]
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

  /// Индекс табуляции.
  #[serde(rename = "@TabIndex")]
  pub tab_index: u32,

  /// Позиция.
  #[serde(rename = "@Position")]
  pub position: u32,

  /// Имя для валидации.
  #[serde(rename = "@ValidateName")]
  pub validate_name: String,

  /// Идентификатор родительского элемента.
  /// Это число, но иногда приходит как `ParentID=""`, считаем что родитель отсутствует.
  #[serde(rename = "@ParentID", default, with = "empty_string_as_none")]
  pub parent_id: Option<i64>,

  /// ТБП значения.
  #[serde(rename = "@ClassID", default)]
  pub class_id: Option<String>,

  /// Зависимость.
  #[serde(rename = "@Depend", default)]
  pub depend: Option<i64>,

  /// Свойства
  #[serde(rename = "@Properties", default)]
  pub properties: Option<String>,

  /// Подсказка.
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
  Subform,
  Line,
  Memo,
  Frame,
  Date,
  Variant,
  Array,
  Panel,
}

/// Спиcок элементов на форме.
#[derive(Debug, Deserialize, Clone)]
pub struct Controls {
  #[serde(rename = "$value", default)]
  pub controls: Vec<Control>,
}

/// Результат выполнения блока `Validate`.
#[derive(Debug, Deserialize, Clone)]
pub struct Validate {
  #[serde(rename = "@DebugText")]
  pub debug_text: String,
  #[serde(rename = "$value", with = "unwrap_list")]
  pub controls_states: Vec<ControlsState>,
}

/// Состояние элемента на форме.
#[derive(Debug, Deserialize, Clone)]
pub struct ControlsState {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Value")]
  pub value: String,
}

/// Результат выполнения блока `Execute`.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "@Result")]
pub struct MethodResult {
  #[serde(rename = "@Value", with = "empty_string_as_none")]
  pub value: Option<i64>,
  #[serde(rename = "$value", with = "unwrap_list")]
  pub controls_states: Vec<ControlsState>,
}

/// Клиент-скрипт
#[derive(Debug, Deserialize, Clone)]
pub struct ClientScript {
  #[serde(rename = "@Text")]
  pub text: String,
}

// Информация об открытой форме.
#[derive(Debug, Deserialize, Clone)]
pub struct MethodFrame {
  #[serde(rename = "@FrameID", default)]
  pub frame_id: Option<i64>,
}

// Группа операций пользователя.
#[derive(Debug, Deserialize, Clone)]
pub struct MethodsGroup {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Name")]
  pub name: String,
}

// Группы операций пользователя
#[derive(Debug, Deserialize, Clone)]
pub struct MethodsGroups {
  #[serde(rename = "$value", default)]
  pub methods_group: Vec<MethodsGroup>,
}

//======================================================================================================================
// Представления и данные
//======================================================================================================================

/// Пункт пользовательского меню.
#[derive(Debug, Deserialize, Clone)]
pub struct UserMenuItem {
  #[serde(rename = "@ID")]
  pub id: i64,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@ViewID")]
  pub view_id: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
}

/// Пользовательское меню.
#[derive(Debug, Deserialize, Clone)]
pub struct UserMenu {
  #[serde(rename = "$value", default)]
  pub user_menu_items: Vec<UserMenuItem>,
}

/// Данные представления.
#[derive(Debug, Deserialize, Clone)]
pub struct ViewData {
  #[serde(rename = "$value", default)]
  pub row: Vec<Row>,
}

/// Строка данных представления.
#[derive(Debug, Deserialize, Clone)]
pub struct Row {
  #[serde(rename = "$value", default)]
  pub row_item: Vec<RowItem>,
}

/// Значение колонки в строке.
#[derive(Debug, Deserialize, Clone)]
pub struct RowItem {
  #[serde(rename = "@ColumnName")]
  pub column_name: String,
  #[serde(rename = "@Value")]
  pub value: String,
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

/// Базовый тип данных колонки.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ColumnBase {
  Memo,
  Date,
  String,
  Number,
  Boolean,
  Reference,
  Collection,
}

/// Выравнивание.
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

/// Видимость.
#[derive(Debug, Clone, Deserialize)]
#[repr(u8)]
pub enum Invisible {
  #[serde(rename = "0")]
  Visible = 0,
  #[serde(rename = "2")]
  Hidden = 2,
}

/// Логирование.
#[derive(Debug, Clone, Deserialize)]
pub enum Logging {
  #[serde(rename = "0")]
  None,
  #[serde(rename = "D")]
  Delete,
}

/// Список колонок.
#[derive(Debug, Deserialize, Clone)]
pub struct Columns {
  #[serde(rename = "$value", default)]
  pub body: Vec<Column>,
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

/// Список представлений.
#[derive(Debug, Deserialize, Clone)]
pub struct Views {
  #[serde(rename = "$value", default)]
  pub body: Vec<View>,
}

//======================================================================================================================
// Навигация, справочники и меню
//======================================================================================================================

/// Список справочников.
#[derive(Debug, Deserialize, Clone)]
pub struct Guides {
  #[serde(rename = "$value", default)]
  pub body: Vec<Class>,
}

/// Группа справочников.
#[derive(Debug, Deserialize, Clone)]
pub struct GuidesGroup {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
}

/// Список групп справочников.
#[derive(Debug, Deserialize, Clone)]
pub struct GuidesGroups {
  #[serde(rename = "$value", default)]
  pub body: Vec<GuidesGroup>,
}

/// Список cправочников.
#[derive(Debug, Deserialize, Clone)]
pub struct Classes {
  #[serde(rename = "$value", default)]
  pub body: Vec<Class>,
}

/// Список ТПБ (не справочников).
#[derive(Debug, Deserialize, Clone)]
pub struct Types {
  #[serde(rename = "$value", default)]
  pub body: Vec<Class>,
}

/// Описание ТБП (типа).
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

/// Тип справочника.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BaseClassID {
  Structure,
}

//======================================================================================================================
// Блокировки
//======================================================================================================================

/// Результат блокировки экземпляра
#[derive(Debug, Deserialize, Clone)]
pub struct LockResult {
  #[serde(rename = "@Message", default)]
  pub message: Option<String>,
}

/// Флаги с тремя состояниями: 0 (выключен), 1 (включен), 2 (специальный/альтернативный)
/// Хранятся как массив из 25 значений.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
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
