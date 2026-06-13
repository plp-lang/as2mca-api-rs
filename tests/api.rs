use std::sync::Once;

use as2mca_api::{client::Client, models::Credentials};
use regex::Regex;

static INIT: Once = Once::new();

fn setup_tracing() {
  INIT.call_once(|| {
    let _ = tracing_subscriber::fmt()
      .pretty()
      .with_test_writer()
      .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
      .try_init();
  });
}

#[tokio::test]
async fn auth() {
  setup_tracing();

  let api_url = std::env::var("2MCA_API_URL").unwrap_or_else(|_| "http://localhost:3000/platform2mca".to_string());
  let username = std::env::var("2MCA_API_USERNAME").unwrap_or_else(|_| "test".to_string());
  let password = std::env::var("2MCA_API_PASSWORD").unwrap_or_else(|_| "test".to_string());

  let client = Client::builder().base_url(api_url).build().unwrap();

  let res = client.authbasic(&Credentials { username, password }).await;
  assert!(res.is_ok());

  let session_id = res.unwrap();
  let hex32_regex = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
  assert!(
    hex32_regex.is_match(session_id.as_str()),
    "session_id '{}' is not a valid 32-char hex string",
    session_id.as_str()
  );

  // ---

  let res = client.session_init(true).await;
  assert!(res.is_ok());

  let debug_pipe_names = res.unwrap().debug_pipe_name;
  let pipe_regex = Regex::new(r"^debug\$\d{10}$").unwrap();
  assert!(
    pipe_regex.is_match(debug_pipe_names.as_str()),
    "debug_pipe_name '{}' does not match pattern 'debug$' + 10 digits",
    debug_pipe_names.as_str()
  );

  // ---

  let res = client.session_deinit(&session_id).await;
  assert!(res.is_ok());
}
