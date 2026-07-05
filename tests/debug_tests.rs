use rstest::rstest;

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_debug(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ref debug_pipe_name,
  } = ctx.await;

  client.pipe_text_get(session_id, debug_pipe_name).await.unwrap();

  client.debug_text_get(session_id, "B").await.unwrap();
}
