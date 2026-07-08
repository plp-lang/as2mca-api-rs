/// Модуль для десериализации пустой строки как отсутствие числа.
pub mod empty_string_as_number {
  use serde::{self, Deserialize, Deserializer, Serializer};

  /// # Errors
  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
  where
    D: Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
  {
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
      None => Ok(None),
      Some(s) if s.is_empty() => Ok(None),
      Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
    }
  }

  /// # Errors
  pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
    T: std::fmt::Display,
  {
    match value {
      Some(v) => serializer.serialize_str(&v.to_string()),
      None => serializer.serialize_str(""),
    }
  }
}

/// Модуль для десериализации строк `"true"` / `"false"` в `bool`.
pub mod string_bool_as_bool {
  use serde::{self, Deserialize, Deserializer};

  /// # Errors
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

/// Модуль для десериализации строк `"1"` / `"0"` в `bool`.
pub mod number_as_bool {
  use serde::{self, Deserialize, Deserializer};

  /// # Errors
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

/// Модуль для развертывания массивов.
pub mod unwrap_list {
  use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

  /// # Errors
  pub fn serialize<'a, T, S>(items: &'a [T], serializer: S) -> Result<S::Ok, S::Error>
  where
    T: Serialize + 'a,
    S: Serializer,
  {
    #[derive(Serialize)]
    struct Wrapper<'b, U>
    where
      U: Serialize + 'b,
    {
      #[serde(rename = "$value", skip_serializing_if = "<[_]>::is_empty")]
      items: &'b [U],
    }
    Wrapper { items }.serialize(serializer)
  }

  /// # Errors
  pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
  where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
  {
    #[derive(Deserialize)]
    #[serde(bound(deserialize = "T: Deserialize<'de>"))]
    struct Wrapper<T> {
      #[serde(rename = "$value", default)]
      items: Vec<T>,
    }
    let wrapper = Wrapper::deserialize(deserializer)?;
    Ok(wrapper.items)
  }
}
