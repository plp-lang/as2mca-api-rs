use rstest::rstest;

use crate::common::ctx::{Context, ctx};
use as2mca_api::requests::{ClassInfo, MethodValidate, MethodValidateDefault, ValidateType, ViewDataGetCancelable};

mod common;

#[rstest]
#[case("USER", "VW_CRIT_USER", "NEW#AUTO")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_method(
  #[future] ctx: Context,
  #[case] class_short_name: &str,
  #[case] view_short_name: &str,
  #[case] method_short_name: &str,
) {
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
      body: None,
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

  let methods = client.class_methods_get(session_id, class_short_name).await.unwrap();
  assert!(!methods.is_empty());

  let method = methods.iter().find(|v| v.short_name == method_short_name).unwrap();
  let method_id = method.id;

  let frame_id = client.method_begin(session_id, method_id).await.unwrap();
  assert!(frame_id == 0);

  let params = client.method_parameters_get(session_id, method_id).await.unwrap();
  assert!(!params.is_empty());

  let _ = client.method_variables_get(session_id, method_id).await.unwrap();

  let controls = client.method_controls_get(session_id, method_id).await.unwrap();
  assert!(!controls.is_empty());

  let class_info: Vec<ClassInfo> = params.iter().map(|p| ClassInfo { class_id: &p.class_id }).collect();
  assert!(!class_info.is_empty());

  let classes = client.classes_get(session_id, &class_info).await.unwrap();
  assert!(!classes.is_empty());

  client
    .method_validate_default(&MethodValidateDefault {
      session_id,
      method_id,
      info: "",
      do_commit: true,
      object_id: Some(object_id),
      class_id: class_short_name,
      debug_level: 10,
      is_called_from_another_method: true,
      read_only: false,
      get_debug_text: true,
      optimized_grid_updates: true,
    })
    .await
    .unwrap();

  client
    .method_validate(&MethodValidate {
      session_id,
      method_id,
      r#type: ValidateType::Validate,
      info: "P_NAME",
      do_commit: true,
      get_debug_text: true,
      optimized_grid_updates: true,
    })
    .await
    .unwrap();

  client.method_end(session_id, frame_id).await.unwrap();
}
