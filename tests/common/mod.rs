use as2mca_api::{
  client::Client,
  models::{
    requests::{Credentials, DebugPipeName, SessionId},
    responses::Session,
  },
};
use regex::Regex;
use std::sync::Once;

static INIT: Once = Once::new();

#[allow(clippy::missing_panics_doc)]
pub async fn setup() -> (Client, SessionId, DebugPipeName) {
  INIT.call_once(|| {
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    let format = std::env::var("AS2MCA_API_LOG_FORMAT").map_or_else(|_| "pretty".to_string(), |s| s.to_lowercase());
    match format.as_str() {
      "pretty" => {
        tracing_subscriber::fmt().pretty().with_env_filter(filter).init();
      }
      "compact" => {
        tracing_subscriber::fmt().compact().with_env_filter(filter).init();
      }
      "json" => {
        tracing_subscriber::fmt().json().with_env_filter(filter).init();
      }
      _ => panic!("unknown logger format"),
    }
  });

  let api_url = std::env::var("AS2MCA_API_URL").unwrap_or_else(|_| "http://localhost:3000/platform2mca/".to_string());
  let username = std::env::var("AS2MCA_API_USERNAME").unwrap_or_else(|_| "test".to_string());
  let password = std::env::var("AS2MCA_API_PASSWORD").unwrap_or_else(|_| "test".to_string());

  let client = Client::new(api_url).expect("Failed to create client");

  // Аутентификация
  let session_id = client
    .authbasic(&Credentials { username, password })
    .await
    .expect("Authbasic failed");

  // Валидация session_id
  let hex32_regex = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
  assert!(
    hex32_regex.is_match(session_id.as_str()),
    "session_id '{}' is not a valid 32-char hex string",
    session_id.as_str()
  );

  // Инициализация сессии
  let Session { debug_pipe_name, .. } = client.session_init(None).await.expect("Session init failed");

  // Валидация debug_pipe_name
  let pipe_regex = Regex::new(r"^debug\$\d{10}$").unwrap();
  assert!(
    pipe_regex.is_match(debug_pipe_name.as_str()),
    "debug_pipe_name '{}' does not match pattern 'debug$' + 10 digits",
    debug_pipe_name.as_str()
  );

  (client, session_id, debug_pipe_name)
}
