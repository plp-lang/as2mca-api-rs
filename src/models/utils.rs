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

/// Модуль для сериализации `bool` в строки `"1"` / `"0"` и обратно.
pub mod bool_as_str {
  use serde::{self, Deserialize, Deserializer, Serializer};

  #[allow(clippy::trivially_copy_pass_by_ref)]
  pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(if *value { "1" } else { "0" })
  }

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

/// Модуль для сериализации `bool` в строки `"true"` / `"false"` и обратно.
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

/// Модуль для сериализации `Option<bool>` в строки `"1"` / `"0"` и обратно.
pub mod option_bool_as_str {
  use serde::{self, Deserialize, Deserializer, Serializer};

  #[allow(clippy::ref_option)]
  #[allow(clippy::trivially_copy_pass_by_ref)]
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
