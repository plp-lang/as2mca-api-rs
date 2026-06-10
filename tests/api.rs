use as2mca_api::{api::session::Credentials, client::Client};
use regex::Regex;

#[tokio::test]
async fn auth() {
  let api_url = std::env::var("2MCA_API_URL").unwrap_or_else(|_| "http://localhost:3000/platform2mca".to_string());
  let username = std::env::var("2MCA_API_USERNAME").unwrap_or_else(|_| "test".to_string());
  let password = std::env::var("2MCA_API_PASSWORD").unwrap_or_else(|_| "test".to_string());

  let client = Client::builder().base_url(api_url).build().unwrap();

  let res = client.session().authbasic(&Credentials { username, password }).await;
  assert!(res.is_ok());

  let res = client.session().init(true).await;
  assert!(res.is_ok());

  let session = res.unwrap();

  let hex32_regex = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
  assert!(
    hex32_regex.is_match(&session.session_id),
    "session_id '{}' is not a valid 32-char hex string",
    session.session_id
  );

  let pipe_regex = Regex::new(r"^debug\$\d{10}$").unwrap();
  assert!(
    pipe_regex.is_match(&session.debug_pipe_name),
    "debug_pipe_name '{}' does not match pattern 'debug' + 10 digits",
    session.debug_pipe_name
  );

  let res = client.session().deinit().await;
  assert!(res.is_ok());
}
