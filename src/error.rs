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

  #[error("XML deserialization error: {0}")]
  XmlDeserializeError(#[from] quick_xml::DeError),

  #[error("XML serialization error: {0}")]
  XmlSerializeError(#[from] quick_xml::SeError),
}

pub type Result<T> = std::result::Result<T, Error>;
