use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
  #[serde(rename = "$value")]
  pub body: ResponseKind,
}

#[derive(Debug, Deserialize)]
pub enum ResponseKind {
  Settings {
    #[serde(rename = "$value")]
    body: Vec<Setting>,
  },
  CoreInfo(CoreInfo),
  ServerInfo(ServerInfo),
  Types(Types),
  GuidesGroups {
    #[serde(rename = "$value")]
    body: Vec<GuidesGroup>,
  },
  Session(Session),
  Done(Done),
  Error(Error),
}

#[derive(Debug, Deserialize)]
pub struct ServerInfo {
  #[serde(rename = "@Version")]
  pub version: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Setting {
  #[serde(rename = "@Name")]
  pub name: String,
  #[serde(rename = "@Value")]
  pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Types {
  #[serde(rename = "$value")]
  pub body: Vec<Class>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct GuidesGroup {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@Name")]
  pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Session {
  #[serde(rename = "@ID")]
  pub id: String,
  #[serde(rename = "@DebugPipeName")]
  pub debug_pipe_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Done {}

#[derive(Debug, Deserialize)]
pub struct Error {
  #[serde(rename = "@Text")]
  pub text: String,
  #[serde(rename = "ServerErrorInfo")]
  pub body: ServerErrorInfo,
}

#[derive(Debug, Deserialize)]
pub struct ServerErrorInfo {
  #[serde(rename = "@Text")]
  pub text: String,
}
