use serde::{Deserialize, Serialize};

use crate::models::{DebugPipeName, SessionId, flags::Flags};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "Response")]
pub struct Response<T> {
  #[serde(rename = "$value")]
  pub body: ResponseBody<T>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum ResponseBody<T> {
  Ok(T),
  Error(Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct ObjectClassAndArchiveKey {
  #[serde(rename = "@ClassID")]
  pub class_id: String,
  #[serde(rename = "@ArchiveKey")]
  pub archive_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DebugText {
  #[serde(default, rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PipeText {
  #[serde(default, rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BackwardReferences {
  #[serde(default, rename = "$value")]
  pub body: Vec<BackwardReference>,
}

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

#[derive(Debug, Deserialize, Clone)]
pub struct ViewData {
  #[serde(default, rename = "$value")]
  pub body: Vec<Row>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Row {
  #[serde(default, rename = "$value")]
  pub body: Vec<RowItem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RowItem {
  #[serde(rename = "@ColumnName")]
  pub column_name: String,
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Transitions {}

#[derive(Debug, Deserialize, Clone)]
pub struct States {}

#[derive(Debug, Deserialize, Clone)]
pub struct Columns {
  #[serde(default, rename = "$value")]
  pub body: Vec<Column>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
  #[serde(rename = "@Name")]
  pub name: String,

  #[serde(rename = "@Width")]
  pub width: u32,

  /// TODO: Left = 0, Center = 1, Right = 2
  #[serde(rename = "@Align")]
  pub align: u8,

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

  /// TODO: Visible = 0, Hidden = 2
  #[serde(rename = "@IsInvisible")]
  pub is_invisible: u8,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ColumnBase {
  String,
  Number,
  Date,
  Reference,
  Collection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Logging {
  #[serde(rename = "0")]
  None,
  #[serde(rename = "D")]
  Delete,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Methods {
  #[serde(default, rename = "$value")]
  pub body: Vec<Method>,
}

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

#[derive(Debug, Deserialize, Clone)]
pub struct MethodsGroups {}

#[derive(Debug, Deserialize, Clone)]
pub struct ChildClasses {}

#[derive(Debug, Deserialize, Clone)]
pub struct Views {
  #[serde(default, rename = "$value")]
  pub body: Vec<View>,
}

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

#[derive(Debug, Deserialize, Clone)]
pub struct UserMenu {}

#[derive(Debug, Deserialize, Clone)]
pub struct Guides {
  #[serde(default, rename = "$value")]
  pub body: Vec<GuideClass>,
}

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

#[derive(Debug, Deserialize, Clone)]
pub struct GuidesGroups {
  #[serde(default, rename = "$value")]
  pub body: Vec<GuidesGroup>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuidesGroup {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Types {
  #[serde(default, rename = "$value")]
  pub body: Vec<Class>,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename = "User")]
pub struct CheckResult {
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "User")]
pub struct OptionInfo {
  #[serde(rename = "@Enabled")]
  pub enabled: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "User")]
pub struct UserProfileProperty {
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "User")]
pub struct UserPrivileged {
  #[serde(rename = "@IsPrivileged")]
  pub is_privileged: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NovoAllowedCheckResult {
  #[serde(rename = "@Value")]
  pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@Properties")]
  pub properties: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
  #[serde(default, rename = "$value")]
  pub body: Vec<Setting>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Setting {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Value")]
  pub value: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerInfo {
  #[serde(rename = "@Version")]
  pub version: String,
}

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

#[derive(Debug, Deserialize, Clone)]
pub struct Session {
  #[serde(rename = "@ID")]
  pub session_id: SessionId,
  #[serde(rename = "@DebugPipeName")]
  pub debug_pipe_name: DebugPipeName,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Done {}

#[derive(Debug, Deserialize, Clone)]
pub struct Error {
  #[serde(rename = "@Text")]
  pub text: String,
  #[serde(rename = "ServerErrorInfo")]
  pub body: ServerErrorInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerErrorInfo {
  #[serde(rename = "@Text")]
  pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProtocolInfo {
  #[serde(rename = "@Version")]
  pub version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthenticationURL {
  #[serde(rename = "@URL")]
  pub url: String,
}

pub mod bool_as_str {
  use serde::{self, Deserialize, Deserializer, Serializer};

  /// Сериализация: bool -> "1" / "0"
  ///
  /// # Errors
  /// [`serde::ser::Error`]
  pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(if *value { "1" } else { "0" })
  }

  /// Десериализация: "1" / "0" -> bool
  ///
  /// # Errors
  /// [`serde::de::Error`]
  pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
      "1" => Ok(true),
      "0" => Ok(false),
      _ => Err(serde::de::Error::custom(format!("expected '1' or '0', received '{s}'"))),
    }
  }
}

pub mod option_bool_as_str {
  use serde::{self, Deserialize, Deserializer, Serializer};

  /// Сериализация: Option<bool> -> "1" / "0"
  ///
  /// # Errors
  /// [`serde::ser::Error`]
  pub fn serialize<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match value {
      Some(true) => serializer.serialize_str("1"),
      Some(false) => serializer.serialize_str("0"),
      None => serializer.serialize_none(),
    }
  }

  /// Десериализация: "1" / "0" -> Option<bool>
  ///
  /// # Errors
  /// [`serde::de::Error`]
  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = Option::<String>::deserialize(deserializer)?;

    match value.as_deref() {
      Some("1") => Ok(Some(true)),
      Some("0") => Ok(Some(false)),
      None => Ok(None),
      Some(other) => Err(serde::de::Error::custom(format!(
        "expected '1' or '0', received '{other}'"
      ))),
    }
  }
}
