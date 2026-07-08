use rstest::rstest;

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[case("MEMO", true)]
#[case("UNKNOWN", false)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_class_get(#[future] ctx: Context, #[case] class_short_name: &str, #[case] is_exists: bool) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let class = client.class_get(session_id, class_short_name).await.unwrap();
  assert_eq!(class.is_some(), is_exists);
}
