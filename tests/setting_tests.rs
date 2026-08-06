use as2mca_api::error::Error;
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
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_context_info_get(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;
  client.system_context_info_get(session_id).await.unwrap();
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

#[rstest]
#[case("UNKNOWN", false)]
#[case("SYS_NAME", true)]
#[case("NOVO.MINIMUM_VERSION", true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_info_get(#[future] ctx: Context, #[case] parameter_name: &str, #[case] is_some: bool) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let res = client.system_info_get(session_id, parameter_name).await.unwrap();
  assert_eq!(res.is_some(), is_some);
}

#[rstest]
#[case("SYS_NAME")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_limit_get(#[future] ctx: Context, #[case] limit_name: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client.system_limit_get(session_id, limit_name).await.unwrap();
}

#[rstest]
#[case("IBS_USER", "SYS_VERSION", true)]
#[case("IBS_USER", "USER_CONTEXT", true)]
#[case("IBS_USER", "USER_LOCK_OPEN", true)]
#[case("IBS_USER", "SYS_BUILD_DATE", true)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_context_get(
  #[future] ctx: Context,
  #[case] namespace: &str,
  #[case] attribute_name: &str,
  #[case] is_some: bool,
) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let res = client
    .system_context_get(session_id, namespace, attribute_name)
    .await
    .unwrap();
  assert_eq!(res.is_some(), is_some);
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_application_name_get(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client.system_application_name_get(session_id).await.unwrap();
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_help_system_info_get(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  match client.system_help_system_info_get(session_id).await {
    Ok(count) => {
      assert!(count > 0);
    }
    Err(Error::Api { message, .. }) => assert_eq!(message, "Справка не установлена"),
    Err(err) => panic!("{err:?}"),
  }
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_embedded_interaction_available_check(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client.embedded_interaction_available_check(session_id).await.unwrap();
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_embedded_interaction_required_check(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client.embedded_interaction_required_check(session_id).await.unwrap();
}

#[rstest]
#[case("STATUS")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_embedded_interaction_get_resource(#[future] ctx: Context, #[case] error_response_type: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client
    .embedded_interaction_get_resource(session_id, Some(error_response_type))
    .await
    .unwrap();
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_context_information_available_check(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client.context_information_available_check(session_id).await.unwrap();
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_embedded_interaction_post(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client
    .embedded_interaction_post(session_id, "ExitApplication")
    .await
    .unwrap();
}

#[rstest]
#[case("VER")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn embedded_interaction_get(#[future] ctx: Context, #[case] value: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client.embedded_interaction_get(session_id, value).await.unwrap();
}
