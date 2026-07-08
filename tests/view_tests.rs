use rstest::rstest;

use crate::common::ctx::{Context, ctx};
use as2mca_api::requests::ViewDataGetCancelable;

mod common;

#[rstest]
#[case("USER", "VW_CRIT_USER")]
#[case("CL_PRIV", "VW_CRIT_CL_PRIV")]
#[case("CL_PRIV", "VW_CRIT_CL_PRIV_EXT")]
#[case("FP_TUNE", "VW_CRIT_FP_TUNE_ALL")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_view(#[future] ctx: Context, #[case] class_short_name: &str, #[case] view_short_name: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let views = client.class_views_get(session_id, class_short_name).await.unwrap();
  assert!(!views.is_empty());

  let view = views.iter().find(|v| v.short_name == view_short_name).unwrap();
  let view_id = view.id;

  let columns = client.view_columns_get(session_id, view_id).await.unwrap();
  assert!(!columns.is_empty());

  let data = client
    .view_data_get_cancelable(&ViewDataGetCancelable {
      session_id,
      view_short_name,
      class_id: class_short_name,
      hint: "FIRST_ROWS",
      allow_timestamp_milliseconds: true,
      rows_limit: Some(10),
      object_filter: None,
    })
    .await
    .unwrap();
  assert!(!data.is_empty());
}
