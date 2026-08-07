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
//! - [`string_as_option_bool`] – десериализует строки `"true"`/`"false"` (или `"1"`/`"0"`) в `Option<bool>`.
//! - [`unwrap_list`] – убирает лишний уровень вложенности для списков в XML (используется для `Vec<T>` внутри элемента). Взято из [документации quick-xml](https://docs.rs/quick-xml/latest/quick_xml/de/#element-lists)
//! - [`comma_separated_numbers`] – сериализует `Vec<T>` в строку вида `"1,2,3"` и обратно.
//!
//! # Использование
//! Модули применяются через атрибуты `#[serde(with = "...")]` или `#[serde(deserialize_with = "...")]`.
//! Они уже интегрированы в структуры из `requests` и `responses`, но могут быть полезны и для пользовательских типов.

#![allow(clippy::missing_errors_doc)]

/// Модуль для десериализации пустой строки как отсутствие значения.
pub mod empty_string_as_none {
  use serde::{self, Deserialize, Deserializer};

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
}

/// Модуль для сериализации и десериализации `bool` как строки.
/// Поддерживает: "true", "false", "1", "0".
pub mod string_as_bool {
  use serde::{self, Deserialize, Deserializer, Serializer};

  pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    // Сериализуем сразу как строковый срез, чтобы избежать аллокаций
    let s = if *value { "1" } else { "0" };
    serializer.serialize_str(s)
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
  where
    D: Deserializer<'de>,
  {
    let s = String::deserialize(deserializer)?;
    match s.to_uppercase().as_str() {
      "TRUE" | "1" => Ok(true),
      "FALSE" | "0" => Ok(false),
      _ => Err(serde::de::Error::custom(format!(
        "expected 'true', 'false', '1' or '0', received '{s}'"
      ))),
    }
  }
}

/// Модуль для сериализации и десериализации `Option<bool>` как строки.
/// Поддерживает: "true", "false", "1", "0".
pub mod string_as_option_bool {
  use serde::{self, Deserialize, Deserializer, Serializer};

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
    let opt_s = Option::<String>::deserialize(deserializer)?;
    match opt_s.map(|s| s.to_uppercase()).as_deref() {
      None => Ok(None),
      Some("TRUE" | "1") => Ok(Some(true)),
      Some("FALSE" | "0") => Ok(Some(false)),
      Some(s) => Err(serde::de::Error::custom(format!(
        "expected 'true', 'false', '1', '0', received '{s}'"
      ))),
    }
  }
}

/// Модуль для развертывания массивов.
pub mod unwrap_list {
  use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

  /// Сериализация поддерживает `Vec<T>`, `&[T]`, `&Vec<T>` и любые другие типы,
  /// которые можно привести к срезу через `AsRef<[T]>`.
  pub fn serialize<T, S, V>(value: &V, serializer: S) -> Result<S::Ok, S::Error>
  where
    T: Serialize,
    S: Serializer,
    V: AsRef<[T]> + ?Sized,
  {
    #[derive(Serialize)]
    struct Wrapper<'b, U>
    where
      U: Serialize + 'b,
    {
      #[serde(rename = "$value", skip_serializing_if = "<[_]>::is_empty")]
      items: &'b [U],
    }
    Wrapper { items: value.as_ref() }.serialize(serializer)
  }

  /// Десериализация всегда возвращает `Vec<T>`.
  /// Возврат `&[T]` невозможен, так как промежуточный Wrapper владеет данными (`Vec`),
  /// и мы не можем вернуть ссылку на локальную переменную.
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
