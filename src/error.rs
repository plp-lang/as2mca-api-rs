//! Определения ошибок, которые могут возникнуть при работе с API.
//!
//! Все ошибки объединены в перечисление [`enum@Error`] с соответствующими вариантами.
//! Результат работы большинства методов возвращает [`Result<T>`] – синоним `std::result::Result<T, Error>`.
//!
//! # Категории ошибок
//! - **Сетевые** – [`Error::Http`] – проблемы с соединением, таймауты, некорректные статусы.
//! - **API‑ошибки** – [`Error::Api`] – сервер вернул структурированное сообщение об ошибке.
//! - **Парсинга** – [`Error::XmlDeserializeError`] / [`Error::XmlSerializeError`] – несоответствие XML‑схеме.
//! - **Логические** – [`Error::UnexpectedResponse`] – сервер вернул неожиданный формат ответа.
//! - **Специфические** – [`Error::NotFoundSessionId`] – отсутствует cookie сессии после авторизации.

use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("API request failed: code: {status_code}, message: {message}, details: {details}")]
  Api {
    status_code: reqwest::StatusCode,
    message: String,
    details: String,
  },

  #[error(
    "Unexpected server response (expected: {expected}). The server API has changed and is incompatible with the current library version. Please open an issue in the project repository and include the details below. Received data: {actual}"
  )]
  UnexpectedResponse { expected: String, actual: String },

  #[error("Not found session id")]
  NotFoundSessionId,

  #[error("HTTP request failed: {0}")]
  Http(#[from] reqwest::Error),

  #[error("Url parse error: {0}")]
  UrlParseError(String),

  #[error("Header value error: {0}")]
  InvalidHeaderValue(#[from] InvalidHeaderValue),

  #[error(
    "XML deserialization error: {0}. The server API has changed and is incompatible with the current library version. Please open an issue in the project repository and include the details below."
  )]
  XmlDeserializeError(#[from] quick_xml::DeError),

  #[error(
    "XML serialization error: {0}. The server API has changed and is incompatible with the current library version. Please open an issue in the project repository and include the details below."
  )]
  XmlSerializeError(#[from] quick_xml::SeError),
}

pub type Result<T> = std::result::Result<T, Error>;
