use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response {
  #[serde(rename = "$value")]
  pub body: ResponseKind,
}

#[derive(Debug, Deserialize)]
pub enum ResponseKind {
  Session(Session),
  Done(Done),
  Error(Error),
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
