use rstest::rstest;

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_protocol_info_get(#[future] ctx: Context) {
  let Context { ref client, .. } = ctx.await;
  client.protocol_info_get().await.unwrap();
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_server_version_get(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;
  client.system_server_version_get(session_id).await.unwrap();
}

#[rstest]
#[case("UNKNOWN", None)]
#[case("SHOW_SYSTEM_MENU", Some("YES"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_setting(#[future] ctx: Context, #[case] name: &str, #[case] value: Option<&str>) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let res = client.system_setting_get(session_id, name).await.unwrap();
  assert_eq!(res, value.map(ToString::to_string));

  let settings = client.system_settings_get(session_id).await.unwrap();
  assert!(!settings.is_empty());

  let res = settings.iter().find(|s| s.name == name).and_then(|v| v.value.clone());
  assert_eq!(res, value.map(ToString::to_string));
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_authentication_url_get(#[future] ctx: Context) {
  let Context { ref client, .. } = ctx.await;
  client.authentication_url_get().await.unwrap();
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_novo_allowed_check(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let is_check = client.novo_allowed_check(session_id).await.unwrap();
  assert!(is_check);
}

#[rstest]
#[case("UNKNOWN", false)]
#[case("NAV_SKIN_INTERFACE", true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_option_enabled_check(#[future] ctx: Context, #[case] name: &str, #[case] value: bool) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let res = client.system_option_enabled_check(session_id, name).await.unwrap();
  assert_eq!(res, value);
}
