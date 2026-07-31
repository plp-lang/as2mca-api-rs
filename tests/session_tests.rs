use regex::Regex;
use rstest::rstest;

use as2mca_api::requests::{NetworkInformationSet, SystemNetAddressSet};

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_session(#[future] ctx: Context) {
  let Context {
    ref session_id,
    ref debug_pipe_name,
    ..
  } = ctx.await;

  let session_id_regex = Regex::new(r"^([0-9a-fA-F]{32}|[\w-]+!\d+!\d+)$").unwrap();
  assert!(
    session_id_regex.is_match(session_id.as_str()),
    "session_id '{}' does not match pattern",
    session_id.as_str()
  );

  let pipe_regex = Regex::new(r"^debug\$\d{10}$").unwrap();
  assert!(
    pipe_regex.is_match(debug_pipe_name.as_str()),
    "debug_pipe_name '{}' does not match pattern 'debug$' + 10 digits",
    debug_pipe_name.as_str()
  );
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_user_privileged_get(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let is_privileged = client.system_user_privileged_get(session_id).await.unwrap();
  assert!(is_privileged);
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_user_info_get(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client.user_info_get(session_id).await.unwrap();
}

#[rstest]
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
async fn test_system_user_privileged(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let is_privileged = client.system_user_privileged_get(session_id).await.unwrap();
  assert!(is_privileged);
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_network_information_set(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client
    .network_information_set(&NetworkInformationSet {
      session_id,
      client_name: "my-host",
      client_ip: "192.168.1.100",
      client_user: "john",
      module_name: "MyApp/1.0",
    })
    .await
    .unwrap();
}

#[rstest]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_system_net_address_set(#[future] ctx: Context) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  client
    .system_net_address_set(&SystemNetAddressSet {
      session_id,
      mac_address: "aabbccddeeff",
      ip_address: "192.168.1.100",
    })
    .await
    .unwrap();
}
