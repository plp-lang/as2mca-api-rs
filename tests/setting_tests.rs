use rstest::rstest;

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[case("UNKNOWN", "")]
#[case("SHOW_SYSTEM_MENU", "YES")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_setting(#[future] ctx: Context, #[case] name: &str, #[case] value: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let res = client.system_setting_get(session_id, name).await.unwrap();
  assert_eq!(res, Some(value.to_string()));

  let settings = client.system_settings_get(session_id).await.unwrap();
  assert!(!settings.is_empty());

  let res = settings
    .iter()
    .find(|s| s.name == name)
    .and_then(|v| v.value.clone())
    .unwrap_or(String::new());
  assert_eq!(res, value);
}

#[rstest]
#[case("UNKNOWN", false)]
#[case("NAV_SKIN_INTERFACE", true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_option(#[future] ctx: Context, #[case] name: &str, #[case] value: bool) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let res = client.system_option_enabled_check(session_id, name).await.unwrap();
  assert_eq!(res, value);
}

#[rstest]
#[case("UNKNOWN", false)]
#[case("ADMIN_GRP", true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_belongs_group_check(#[future] ctx: Context, #[case] name: &str, #[case] value: bool) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let res = client.user_belongs_group_check(session_id, name).await.unwrap();
  assert_eq!(res, value);
}

#[rstest]
// #[case("UNKNOWN", "")]
#[case("SESSIONS_PER_USER", "UNLIMITED")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_profile_property(#[future] ctx: Context, #[case] name: &str, #[case] value: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let res = client.user_profile_property_get(session_id, name).await.unwrap();
  assert_eq!(res, value.to_string());
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
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_user_privileged(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let is_privileged = client.system_user_privileged_get(session_id).await.unwrap();
  assert!(is_privileged);
}
