use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Flags(pub u64);

impl Flags {
  #[must_use]
  pub const fn has_flag(self, bit: u32) -> bool {
    (self.0 & (1 << bit)) != 0
  }
}

impl TryFrom<String> for Flags {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    u64::from_str_radix(&value, 2)
      .map(Flags)
      .map_err(|e| format!("Invalid binary flags '{value}': {e}"))
  }
}

impl From<Flags> for String {
  fn from(flags: Flags) -> Self {
    format!("{:025b}", flags.0)
  }
}

/// Модуль для десериализации строк `"true"` / `"false"` в `bool`.
pub mod bool_as_bool {
  use serde::{self, Deserialize, Deserializer};

  pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
      "true" => Ok(true),
      "false" => Ok(false),
      _ => Err(serde::de::Error::custom(format!(
        "expected 'true' or 'false', received '{s}'"
      ))),
    }
  }
}
