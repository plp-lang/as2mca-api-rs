use rstest::rstest;

use as2mca_api::responses::{BackwardReference, ChildClass, ObjectClassAndArchiveKey, State, Transition};

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[case("USER", 8_935_328, Some("USER"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_object_class_and_archive_key_get(
  #[future] ctx: Context,
  #[case] base_class_id: &str,
  #[case] object_id: i64,
  #[case] expected_class_id: Option<&str>,
) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let ObjectClassAndArchiveKey { class_id, .. } = client
    .object_class_and_archive_key_get(session_id, object_id, base_class_id)
    .await
    .unwrap();
  assert_eq!(class_id, expected_class_id.map(String::from));
}

#[rstest]
#[case("USER", 8_935_328, vec![BackwardReference { class_id: "ACCOUNT".to_string(), class_name: "Счета".to_string(), qual: "USER_OP".to_string(), qual_name: "Создатель".to_string() }])]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_object_backward_references_get(
  #[future] ctx: Context,
  #[case] base_class_id: &str,
  #[case] object_id: i64,
  #[case] expected_refs: Vec<BackwardReference>,
) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let refs = client
    .object_backward_references_get(session_id, object_id, base_class_id)
    .await
    .unwrap();
  for expected in &expected_refs {
    assert!(refs.contains(expected));
  }
}

#[rstest]
#[case("VZ_CARDS", vec![Transition { id: 10_552_294, name: "(Слж.) Закрыта -> Создана".to_string(), method_short_name: None, initial_state_id: "ARC".to_string(), final_state_id: "CRT".to_string() }])]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_class_transitions_get(
  #[future] ctx: Context,
  #[case] class_id: &str,
  #[case] expected_trans: Vec<Transition>,
) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let trans = client.class_transitions_get(session_id, class_id).await.unwrap();
  for expected in &expected_trans {
    assert!(trans.contains(expected));
  }
}

#[rstest]
#[case("VZ_CARDS", vec![State { id: "WRK".to_string(), name: "Рабочая".to_string(), index_use: 0 }])]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_class_states_get(#[future] ctx: Context, #[case] class_id: &str, #[case] expected_states: Vec<State>) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let states = client.class_states_get(session_id, class_id).await.unwrap();
  for expected in &expected_states {
    assert!(states.contains(expected));
  }
}

#[rstest]
#[case("VZ_CARDS", false)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_class_need_collection_id_check(#[future] ctx: Context, #[case] class_id: &str, #[case] expected: bool) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let is_need = client
    .class_need_collection_id_check(session_id, class_id)
    .await
    .unwrap();
  assert_eq!(is_need, expected);
}

#[rstest]
#[case("DOCUMENT", vec![ChildClass { id: "ASV_REEST_CLAIM".to_string() }])]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_class_children_get(
  #[future] ctx: Context,
  #[case] class_id: &str,
  #[case] expected_childs: Vec<ChildClass>,
) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let childs = client.class_children_get(session_id, class_id).await.unwrap();
  for expected in &expected_childs {
    assert!(childs.contains(expected));
  }
}

#[rstest]
#[case("MEMO", true)]
#[case("DOCUMENT", true)]
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

#[rstest]
#[case(&["MEMO", "DOCUMENT"])]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_classes_get(#[future] ctx: Context, #[case] classes: &[&str]) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let result = client.classes_get(session_id, classes).await.unwrap();
  assert_eq!(result.len(), classes.len());
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_guides_get(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let result = client.guides_get(session_id).await.unwrap();
  assert!(!result.is_empty());
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_types_get(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let result = client.types_get(session_id).await.unwrap();
  assert!(!result.is_empty());
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_guides_groups_get(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let result = client.guides_groups_get(session_id).await.unwrap();
  assert!(!result.is_empty());
}
