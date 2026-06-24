use core::fmt;

use serde::{Deserialize, Serialize};

pub mod flags;
pub mod requests;
pub mod responses;

#[derive(Debug, Clone)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(String);

impl SessionId {
  #[must_use]
  pub const fn new(id: String) -> Self {
    Self(id)
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for SessionId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<String> for SessionId {
  fn from(s: String) -> Self {
    Self::new(s)
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DebugPipeName(String);

impl DebugPipeName {
  #[must_use]
  pub const fn new(id: String) -> Self {
    Self(id)
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for DebugPipeName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<String> for DebugPipeName {
  fn from(s: String) -> Self {
    Self::new(s)
  }
}
