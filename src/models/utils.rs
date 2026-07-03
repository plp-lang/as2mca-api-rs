use serde::{Deserialize, Serialize};

/// Флаги с тремя состояниями: 0 (выключен), 1 (включен), 2 (специальный/альтернативный)
/// Хранятся как массив из 25 значений.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Flags([u8; 25]);

impl Flags {
  pub const LEN: usize = 25;

  /// Создаёт флаги из массива значений
  #[must_use]
  pub const fn new(values: [u8; 25]) -> Self {
    Self(values)
  }

  /// Получает значение флага по индексу
  #[must_use]
  pub const fn get(&self, index: usize) -> u8 {
    self.0[index]
  }

  /// Проверяет, установлен ли флаг (значение != 0)
  #[must_use]
  pub const fn has_flag(&self, index: usize) -> bool {
    self.0[index] != 0
  }

  /// Проверяет, что флаг имеет конкретное значение
  #[must_use]
  pub const fn is(&self, index: usize, value: u8) -> bool {
    self.0[index] == value
  }
}

impl TryFrom<String> for Flags {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    if value.len() != Self::LEN {
      return Err(format!(
        "Invalid flags length: expected {}, got {}",
        Self::LEN,
        value.len()
      ));
    }

    let mut result = [0u8; 25];
    for (i, c) in value.chars().enumerate() {
      result[i] = match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        _ => {
          return Err(format!("Invalid character '{c}' at position {i} in flags '{value}'"));
        }
      };
    }

    Ok(Self(result))
  }
}

impl From<Flags> for String {
  #[allow(clippy::cast_lossless)]
  fn from(flags: Flags) -> Self {
    flags
      .0
      .iter()
      .map(|&b| char::from_digit(b as u32, 10).unwrap())
      .collect()
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

/// Модуль для десериализации строк `"1"` / `"0"` в `bool`.
pub mod number_as_bool {
  use serde::{self, Deserialize, Deserializer};

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
