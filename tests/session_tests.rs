use regex::Regex;
use rstest::rstest;

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_session(#[future] ctx: Context) {
  let Context {
    ref session_id,
    ref debug_pipe_name,
    ..
  } = ctx.await;

  // Валидация session_id
  let hex32_regex = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
  assert!(
    hex32_regex.is_match(session_id.as_str()),
    "session_id '{}' is not a valid 32-char hex string",
    session_id.as_str()
  );

  // Валидация debug_pipe_name
  let pipe_regex = Regex::new(r"^debug\$\d{10}$").unwrap();
  assert!(
    pipe_regex.is_match(debug_pipe_name.as_str()),
    "debug_pipe_name '{}' does not match pattern 'debug$' + 10 digits",
    debug_pipe_name.as_str()
  );
}
