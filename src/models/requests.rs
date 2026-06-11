use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Request {
  #[serde(rename = "$value")]
  pub body: RequestKind,
}

#[derive(Debug, Serialize)]
pub enum RequestKind {
  SystemSettingsGet {
    #[serde(rename = "@SessionID")]
    session_id: String,
  },
  SystemCoreInfoGet {
    #[serde(rename = "@SessionID")]
    session_id: String,
  },
  SystemServerVersionGet {
    #[serde(rename = "@SessionID")]
    session_id: String,
  },
  TypesGet(TypesGet),
  GuidesGroupsGet(GuidesGroupsGet),
  SessionInit(SessionInit),
  Disconnect(Disconnect),
}

#[derive(Debug, Serialize)]
pub struct TypesGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Serialize)]
pub struct GuidesGroupsGet {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}

#[derive(Debug, Serialize)]
pub struct SessionInit {
  #[serde(rename = "@AliveActiveSession")]
  pub alive_active_session: String,
}

#[derive(Debug, Serialize)]
pub struct Disconnect {
  #[serde(rename = "@SessionID")]
  pub session_id: String,
}
