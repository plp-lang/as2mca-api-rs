use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Request {
  #[serde(rename = "$value")]
  pub body: RequestKind,
}

#[derive(Debug, Serialize)]
pub enum RequestKind {
  SessionInit(SessionInit),
  Disconnect(Disconnect),
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
