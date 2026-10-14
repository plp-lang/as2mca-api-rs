use rstest::rstest;

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[case("USER")]
#[case("CL_ORG")]
#[case("CL_PRIV")]
#[case("DOCUMENT")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_view_columns_and_data(#[future] ctx: Context, #[case] class_short_name: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let views = client.class_views_get(session_id, class_short_name).await.unwrap();
  assert_ne!(views, [] as [as2mca_api::responses::View; 0]);

  for view in views {
    let columns = client.view_columns_get(session_id, view.id).await.unwrap();
    assert_ne!(columns, [] as [as2mca_api::responses::Column; 0]);
  }
}
