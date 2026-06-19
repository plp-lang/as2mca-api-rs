use serde::Serialize;

use crate::models::SessionId;

pub const XML_HEADER: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;

#[derive(Debug, Serialize)]
pub struct Request {
  #[serde(rename = "$value")]
  pub body: RequestKind,
}

#[derive(Debug, Serialize)]
pub enum RequestKind {
  ClassChildrenGet(ClassChildrenGet),
  ClassViewsGet(ClassViewsGet),
  UserMenuGet(UserMenuGet),
  GuidesGet(GuidesGet),
  GuidesGroupsGet(GuidesGroupsGet),
  TypesGet(TypesGet),
  UserBelongsGroupCheck(UserBelongsGroupCheck),
  SystemOptionEnabledCheck(SystemOptionEnabledCheck),
  UserProfilePropertyGet(UserProfilePropertyGet),
  NetworkInformationSet(NetworkInformationSet),
  SystemUserPrivilegedGet(SystemUserPrivilegedGet),
  NovoAllowedCheck(NovoAllowedCheck),
  SystemNetAddressSet(SystemNetAddressSet),
  UserInfoGet(UserInfoGet),
  AuthenticationURLGet(AuthenticationURLGet),
  ProtocolInfoGet(ProtocolInfoGet),
  SystemSettingsGet(SystemSettingsGet),
  SystemCoreInfoGet(SystemCoreInfoGet),
  SystemServerVersionGet(SystemServerVersionGet),
  SessionInit(SessionInit),
  Disconnect(Disconnect),
}

#[derive(Debug, Serialize, Clone)]
pub struct ClassChildrenGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClassViewsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@ClassID")]
  pub class_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserMenuGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize, Clone)]
pub struct GuidesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize, Clone)]
pub struct GuidesGroupsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize, Clone)]
pub struct TypesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserBelongsGroupCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@GroupID")]
  pub group_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct SystemOptionEnabledCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@OptionName")]
  pub option_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserProfilePropertyGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@PropertyName")]
  pub property_name: String,
}

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

#[derive(Debug, Serialize, Clone)]
pub struct SystemUserPrivilegedGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize, Clone)]
pub struct NovoAllowedCheck {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize, Clone)]
pub struct SystemNetAddressSet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
  #[serde(rename = "@MACAddress")]
  pub mac_address: String,
  #[serde(rename = "@IPAddress")]
  pub ip_address: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize)]
pub struct AuthenticationURLGet {}

#[derive(Debug, Serialize)]
pub struct ProtocolInfoGet {}

#[derive(Debug, Serialize)]
pub struct SystemSettingsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize)]
pub struct SystemCoreInfoGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize)]
pub struct SystemServerVersionGet {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}

#[derive(Debug, Serialize)]
pub struct SessionInit {
  #[serde(rename = "@AliveActiveSession", skip_serializing_if = "Option::is_none")]
  pub alive_active_session: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct Disconnect {
  #[serde(rename = "@SessionID")]
  pub session_id: SessionId,
}
