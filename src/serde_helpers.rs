//! Вспомогательные модули для сериализации/десериализации с помощью `serde`.
//!
//! Эти модули используются внутри структур запросов и ответов для обработки нестандартных форматов:
//! - преобразование пустой строки в `None` для чисел;
//! - разбор строк `"true"`/`"false"` как `bool`;
//! - сериализация/десериализация списков;
//! - преобразование массива чисел в строку через запятую.
//!
//! # Модули
//! - [`empty_string_as_none`] – позволяет полю типа `Option<T>` принимать пустую строку как `None`.
//! - [`string_as_bool`] – десериализует строки `"true"`/`"false"` (или `"1"`/`"0"`) в `bool`.
//! - [`unwrap_list`] – убирает лишний уровень вложенности для списков в XML (используется для `Vec<T>` внутри элемента). Взято из [документации quick-xml](https://docs.rs/quick-xml/latest/quick_xml/de/#element-lists)
//! - [`comma_separated_numbers`] – сериализует `Vec<T>` в строку вида `"1,2,3"` и обратно.
//!
//! # Использование
//! Модули применяются через атрибуты `#[serde(with = "...")]` или `#[serde(deserialize_with = "...")]`.
//! Они уже интегрированы в структуры из `requests` и `responses`, но могут быть полезны и для пользовательских типов.

#![allow(clippy::missing_errors_doc)]

/// Модуль для десериализации пустой строки как отсутствие значения.
pub mod empty_string_as_none {
  use serde::{self, Deserialize, Deserializer, Serializer};

  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
  where
    D: Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
  {
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
      None => Ok(None),
      Some(ref s) if s.is_empty() => Ok(None),
      Some(s) => s.parse::<T>().map(Some).map_err(serde::de::Error::custom),
    }
  }

  pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
    T: std::fmt::Display,
  {
    match value {
      Some(v) => serializer.collect_str(v),
      None => serializer.serialize_str(""),
    }
  }
}

/// Модуль для десериализации строк в `bool`.
pub mod string_as_bool {
  use serde::{self, Deserialize, Deserializer};

  pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
      "true" | "1" => Ok(true),
      "false" | "0" => Ok(false),
      _ => Err(serde::de::Error::custom(format!(
        "expected 'true' or 'false', received '{s}'"
      ))),
    }
  }
}

/// Модуль для развертывания массивов.
pub mod unwrap_list {
  use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

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

/// Модуль для сериализации и десериализации массива чисел в строку через запятую.
/// Сериализация `Vec<T>` в строку "1,2,3"
/// Десериализация строки "1,2,3" или "" в `Vec<T>`
pub mod comma_separated_numbers {
  use serde::{self, Deserialize, Deserializer, Serializer};
  use std::fmt::Display;
  use std::str::FromStr;

  pub fn serialize<S, T>(vec: &[T], serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
    T: Display,
  {
    let s = vec.iter().map(ToString::to_string).collect::<Vec<_>>().join(",");
    serializer.serialize_str(&s)
  }

  pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
  where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: Display,
  {
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
      return Ok(Vec::new());
    }
    s.split(',')
      .map(str::trim)
      .filter(|s| !s.is_empty())
      .map(|part| part.parse::<T>().map_err(serde::de::Error::custom))
      .collect()
  }
}
