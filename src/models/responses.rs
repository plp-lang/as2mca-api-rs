use serde::Deserialize;

use crate::models::{DebugPipeName, SessionId};

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
pub struct Columns {
  #[serde(default, rename = "$value")]
  pub body: Vec<Column>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Column {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Width")]
  pub width: String,
  #[serde(rename = "@Align")]
  pub align: String,
  #[serde(rename = "@Position")]
  pub position: String,
  #[serde(rename = "@Qual")]
  pub qual: String,
  #[serde(rename = "@Alias")]
  pub alias: String,
  #[serde(rename = "@Base")]
  pub base: String,
  #[serde(rename = "@IsEditable")]
  pub is_editable: Option<String>,
  #[serde(rename = "@IsSizeable")]
  pub is_sizeable: String,
  #[serde(rename = "@IsCellStyle")]
  pub is_cell_style: String,
  #[serde(rename = "@IsInvisible")]
  pub is_invisible: String,
  #[serde(rename = "@TargetClassID")]
  pub target_class_id: Option<String>,
  #[serde(rename = "@ReferenceType")]
  pub reference_type: Option<String>,
  #[serde(rename = "@Logging")]
  pub logging: Option<String>,
  #[serde(rename = "@AbilityPerformOperation")]
  pub ability_perform_operation: Option<String>,
  #[serde(rename = "@ReferenceID")]
  pub reference_id: Option<String>,
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

#[derive(Debug, Deserialize, Clone)]
pub struct View {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@ShortName")]
  pub short_name: String,
  #[serde(rename = "@IsDefault")]
  pub is_default: String,
  #[serde(rename = "@CellStyleScript")]
  pub cell_style_script: Option<String>,
  #[serde(rename = "@Properties")]
  pub properties: String,
  #[serde(rename = "@Distance")]
  pub distance: String,
  #[serde(rename = "@FilterMethodShortName")]
  pub filter_method_short_name: Option<String>,
  #[serde(rename = "@FilterMethodProperties")]
  pub filter_method_properties: Option<String>,
  #[serde(rename = "@ObjectRights")]
  pub object_rights: String,
  #[serde(rename = "@ToPrinter")]
  pub to_printer: String,
  #[serde(rename = "@ToFile")]
  pub to_file: String,
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
  pub flags: String,
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
  #[serde(rename = "@IsKernelType")]
  pub is_kernel_type: String,
  #[serde(rename = "@ClassInterface")]
  pub class_interface: String,
  #[serde(rename = "@IsAccessible")]
  pub is_accessible: String,
  #[serde(rename = "@Flags")]
  pub flags: String,
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
  pub id: SessionId,
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
