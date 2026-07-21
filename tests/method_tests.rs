use rstest::rstest;

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[case("AC_FIN")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_methods(#[future] ctx: Context, #[case] class_short_name: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let methods = client.class_methods_get(session_id, class_short_name).await.unwrap();
  for method in methods {
    let _script = client.method_client_script_get(session_id, method.id).await.unwrap();
    let _controls = client
      .method_controls_get(session_id, method.form_id.unwrap_or(method.id))
      .await
      .unwrap();
    let _params = client.method_parameters_get(session_id, method.id).await.unwrap();
    let _vars = client.method_variables_get(session_id, method.id).await.unwrap();
  }
}

// #[rstest]
// #[case("FP_TUNE", "VW_CRIT_FP_TUNE_ALL", "NEW#AUTO")]
// #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
// async fn test_methods(
//   #[future] ctx: Context,
//   #[case] class_short_name: &str,
//   #[case] view_short_name: &str,
//   #[case] method_short_name: &str,
// ) {
//   let Context {
//     ref client,
//     ref session_id,
//     ..
//   } = ctx.await;

//   // Получаем список представлений ТБП
//   let views = client.class_views_get(session_id, class_short_name).await.unwrap();
//   assert!(!views.is_empty());

//   let view = views.iter().find(|v| v.short_name == view_short_name).unwrap();
//   let view_short_name = view.short_name.as_str();

//   // Берем самый первый экземпляр представления
//   let data = client
//     .view_data_get_cancelable(&ViewDataGetCancelable {
//       session_id,
//       view_short_name,
//       class_id: class_short_name,
//       rows_limit: Some(1),
//       ..Default::default()
//     })
//     .await
//     .unwrap();
//   assert!(!data.is_empty());

//   // Ищем `ID` первого экземпляра представления
//   let object_id = data
//     .iter()
//     .find_map(|row| {
//       row
//         .row_item
//         .iter()
//         .find(|item| item.column_name == "ID")
//         .and_then(|item| item.value.parse::<i64>().ok())
//     })
//     .unwrap();

//   let methods = client.class_methods_get(session_id, class_short_name).await.unwrap();
//   assert!(!methods.is_empty());

//   let method = methods.iter().find(|v| v.short_name == method_short_name).unwrap();
//   let form_id = method.form_id.unwrap_or(method.id);
//   let method_id = method.id;

//   let _ = client.method_client_script_get(session_id, method_id).await.unwrap();

//   let frame_id = client.method_begin(session_id, method_id).await.unwrap();
//   assert!(frame_id == 0);

//   let params = client.method_parameters_get(session_id, method_id).await.unwrap();
//   assert!(!params.is_empty());

//   let _ = client.method_variables_get(session_id, method_id).await.unwrap();

//   let controls = client.method_controls_get(session_id, form_id).await.unwrap();
//   assert!(!controls.is_empty());

//   client
//     .method_validate_default(&MethodValidateDefault {
//       session_id,
//       method_id,
//       object_id: &[object_id],
//       ..Default::default()
//     })
//     .await
//     .unwrap();

//   client.method_end(session_id, frame_id).await.unwrap();
// }

// #[rstest]
// #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
// async fn test_method_new_auto_fp_tune(#[future] ctx: Context) {
//   const CLASS_SHORT_NAME: &str = "FP_TUNE";
//   const METHOD_CREATE_SHORT_NAME: &str = "NEW#AUTO";
//   const METHOD_DELETE_SHORT_NAME: &str = "DELETE#AUTO";

//   let Context {
//     ref client,
//     ref session_id,
//     ..
//   } = ctx.await;

//   let methods = client.class_methods_get(session_id, CLASS_SHORT_NAME).await.unwrap();
//   assert!(!methods.is_empty());

//   let method_create = methods
//     .iter()
//     .find(|v| v.short_name == METHOD_CREATE_SHORT_NAME)
//     .unwrap();
//   let method_create_id = method_create.id;

//   let frame_id = client.method_begin(session_id, method_create_id).await.unwrap();
//   assert!(frame_id == 0);

//   client
//     .method_validate_default(&MethodValidateDefault {
//       session_id,
//       method_id: method_create_id,
//       class_id: CLASS_SHORT_NAME,
//       ..Default::default()
//     })
//     .await
//     .unwrap();

//   // Удаляем

//   let method_delete = methods
//     .iter()
//     .find(|v| v.short_name == METHOD_DELETE_SHORT_NAME)
//     .unwrap();
//   let method_delete_id = method_delete.id;

//   let frame_id = client.method_begin(session_id, method_delete_id).await.unwrap();
//   assert!(frame_id == 0);

//   client
//     .method_validate_default(&MethodValidateDefault {
//       session_id,
//       method_id: method_delete_id,
//       class_id: CLASS_SHORT_NAME,
//       object_id: &[object_id],
//       ..Default::default()
//     })
//     .await
//     .unwrap();

//   client
//     .method_execute(&MethodExecute {
//       session_id,
//       method_id: method_delete_id,
//       ..Default::default()
//     })
//     .await
//     .unwrap();

//   client.method_end(session_id, frame_id).await.unwrap();
// }
