use rstest::rstest;

use crate::common::ctx::{Context, ctx};
use as2mca_api::requests::{Object, ViewDataGetCancelable};

mod common;

#[rstest]
#[case("USER", "VW_CRIT_USER")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_lock(#[future] ctx: Context, #[case] class_short_name: &str, #[case] view_short_name: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  // Получаем список представлений ТБП
  let views = client.class_views_get(session_id, class_short_name).await.unwrap();
  assert!(!views.is_empty());

  let view = views.iter().find(|v| v.short_name == view_short_name).unwrap();
  let view_short_name = view.short_name.as_str();

  // Берем самый первый экземпляр представления
  let data = client
    .view_data_get_cancelable(&ViewDataGetCancelable {
      session_id,
      view_short_name,
      class_id: class_short_name,
      hint: "FIRST_ROWS",
      allow_timestamp_milliseconds: true,
      rows_limit: Some(1),
      object_filter: None,
    })
    .await
    .unwrap();
  assert!(!data.is_empty());

  // Ищем `ID` первого экземпляра представления
  let object_id = data
    .iter()
    .find_map(|row| {
      row
        .row_item
        .iter()
        .find(|item| item.column_name == "ID")
        .and_then(|item| item.value.parse::<i64>().ok())
    })
    .unwrap();

  let err = client
    .objects_lock(
      session_id,
      &[Object {
        id: object_id,
        class_id: class_short_name,
      }],
    )
    .await
    .unwrap();
  assert!(err.is_none());

  client.objects_unlock(session_id, true).await.unwrap();
}
