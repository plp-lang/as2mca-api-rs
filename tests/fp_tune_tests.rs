use as2mca_api::requests::{
  CaseInsensitiveFilter, ControlState, Filter, MethodExecute, MethodValidate, MethodValidateDefault, Object,
  ObjectFilter, UserFilter, ViewDataGetCancelable,
};
use rstest::rstest;

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_fp_tune(#[future] ctx: Context) {
  const TEST_OBJECT_VALUE: &str = "AS2MCA_API_TEST_OBJECT";
  const CLASS_SHORT_NAME: &str = "FP_TUNE";
  const VIEW_SHORT_NAME: &str = "VW_CRIT_FP_TUNE_ALL";
  const METHOD_CREATE_SHORT_NAME: &str = "NEW#AUTO";
  const METHOD_DELETE_SHORT_NAME: &str = "DELETE#AUTO";

  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  // Поиск необходимых для теста операций и представлений

  let views = client.class_views_get(session_id, CLASS_SHORT_NAME).await.unwrap();
  let _view_id = views.iter().find(|v| v.short_name == VIEW_SHORT_NAME).unwrap().id;

  let methods = client.class_methods_get(session_id, CLASS_SHORT_NAME).await.unwrap();
  let method_create_id = methods
    .iter()
    .find(|m| m.short_name == METHOD_CREATE_SHORT_NAME)
    .unwrap()
    .id;
  let method_delete_id = methods
    .iter()
    .find(|m| m.short_name == METHOD_DELETE_SHORT_NAME)
    .unwrap()
    .id;

  // Поиск тестового экземпляра, удаляем если нашли

  let data = client
    .view_data_get_cancelable(&ViewDataGetCancelable {
      session_id,
      view_short_name: VIEW_SHORT_NAME,
      class_id: CLASS_SHORT_NAME,
      user_filter: Some(UserFilter {
        extra_filter: None,
        filters: vec![Filter::And(vec![Filter::CaseInsensitive(CaseInsensitiveFilter {
          column_name: "C_2",
          operator: "=",
          value: Some(TEST_OBJECT_VALUE),
        })])],
      }),
      ..Default::default()
    })
    .await
    .unwrap();

  let parsed_data = data.first().and_then(|row| {
    let row_id = row.row_item.iter().find(|v| v.column_name == "ID")?;
    let row_code = row.row_item.iter().find(|v| v.column_name == "C_2")?;

    let object_id = row_id.value.parse::<i64>().ok()?;
    let object_code = row_code.value.clone();

    Some((object_id, object_code))
  });

  if let Some((object_id, object_code)) = parsed_data
    && object_code == TEST_OBJECT_VALUE
  {
    let frame_id = client.method_begin(session_id, method_delete_id).await.unwrap();
    client
      .method_validate_default(&MethodValidateDefault {
        session_id,
        class_id: CLASS_SHORT_NAME,
        method_id: method_delete_id,
        object_id: &[object_id],
        ..Default::default()
      })
      .await
      .unwrap();
    client
      .method_execute(&MethodExecute {
        session_id,
        method_id: method_delete_id,
        ..Default::default()
      })
      .await
      .unwrap();
    let prev_frame_id = client.method_end(session_id, frame_id).await.unwrap();
    assert_eq!(prev_frame_id, None);
  }

  // Создаем тестовый экземпляр

  let frame_id = client.method_begin(session_id, method_create_id).await.unwrap();

  client
    .method_validate_default(&MethodValidateDefault {
      session_id,
      class_id: CLASS_SHORT_NAME,
      method_id: method_create_id,
      ..Default::default()
    })
    .await
    .unwrap();

  client
    .method_validate(&MethodValidate {
      session_id,
      method_id: method_create_id,
      info: "%PARAM%.P_CODE",
      controls_states: vec![ControlState {
        id: 17_007_818,
        value: TEST_OBJECT_VALUE,
      }],
      ..Default::default()
    })
    .await
    .unwrap();

  client
    .method_validate(&MethodValidate {
      session_id,
      method_id: method_create_id,
      info: "%PARAM%.P_NAME",
      controls_states: vec![ControlState {
        id: 17_007_820,
        value: TEST_OBJECT_VALUE,
      }],
      ..Default::default()
    })
    .await
    .unwrap();

  client
    .method_validate(&MethodValidate {
      session_id,
      method_id: method_create_id,
      info: "%PARAM%.P_GROUP_ID",
      controls_states: vec![ControlState {
        id: 17_007_839,
        value: TEST_OBJECT_VALUE,
      }],
      ..Default::default()
    })
    .await
    .unwrap();

  client
    .method_validate(&MethodValidate {
      session_id,
      method_id: method_create_id,
      info: "%VAR%.V_VAL_TYPE.0",
      controls_states: vec![ControlState {
        id: 17_007_844,
        value: "4",
      }],
      ..Default::default()
    })
    .await
    .unwrap();

  client
    .method_validate(&MethodValidate {
      session_id,
      method_id: method_create_id,
      info: "%VAR%.V_VAL_BOOL.0",
      controls_states: vec![ControlState {
        id: 17_007_835,
        value: "1",
      }],
      ..Default::default()
    })
    .await
    .unwrap();

  let res = client
    .method_execute(&MethodExecute {
      session_id,
      method_id: method_create_id,
      ..Default::default()
    })
    .await
    .unwrap();
  let object_id = res.value.unwrap().parse::<i64>().unwrap();

  let prev_frame_id = client.method_end(session_id, frame_id).await.unwrap();
  assert_eq!(prev_frame_id, None);

  // Проверяем наличие созданого тестового экземпляра

  let data = client
    .view_data_get_cancelable(&ViewDataGetCancelable {
      session_id,
      class_id: CLASS_SHORT_NAME,
      view_short_name: VIEW_SHORT_NAME,
      object_filter: Some(ObjectFilter { object_id }),
      ..Default::default()
    })
    .await
    .unwrap();

  let row = data.first().unwrap();
  let row_code = row.row_item.iter().find(|v| v.column_name == "C_2").unwrap();
  let object_code = row_code.value.clone();
  assert_eq!(&object_code, TEST_OBJECT_VALUE);

  // Блокируем тестовый экземпляр

  client
    .objects_lock(
      session_id,
      &[Object {
        id: object_id,
        class_id: CLASS_SHORT_NAME,
      }],
    )
    .await
    .unwrap();

  // Удаляем тестовый экземпляр

  let frame_id = client.method_begin(session_id, method_delete_id).await.unwrap();

  client
    .method_validate_default(&MethodValidateDefault {
      session_id,
      class_id: CLASS_SHORT_NAME,
      method_id: method_delete_id,
      object_id: &[object_id],
      ..Default::default()
    })
    .await
    .unwrap();

  client
    .method_execute(&MethodExecute {
      session_id,
      method_id: method_delete_id,
      ..Default::default()
    })
    .await
    .unwrap();

  let prev_frame_id = client.method_end(session_id, frame_id).await.unwrap();
  assert_eq!(prev_frame_id, None);

  // Снимаем блокировку с экземпляра

  client.objects_unlock(session_id, None).await.unwrap();
}
