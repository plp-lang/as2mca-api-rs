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

#[derive(Debug, Deserialize, Clone)]
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
