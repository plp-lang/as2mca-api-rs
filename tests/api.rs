use std::sync::Once;

use as2mca_api::{
  client::Client,
  models::{
    Credentials,
    requests::{
      ClassChildrenGet, ClassMethodsGroupsUserGet, ClassNeedCollectionIDCheck, ClassStatesGet, ClassTransitionsGet,
      NetworkInformationSet, ObjectBackwardReferencesGet, SystemNetAddressSet, SystemOptionEnabledCheck,
      UserBelongsGroupCheck, UserProfilePropertyGet, ViewColumnsGet, ViewDataGetCancelable,
    },
    responses::{CheckResult, Session},
  },
};
use regex::Regex;

static INIT: Once = Once::new();

fn setup_tracing() {
  INIT.call_once(|| {
    let _ = tracing_subscriber::fmt()
      .pretty()
      .with_test_writer()
      .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
      .try_init();
  });
}

#[allow(clippy::too_many_lines)]
#[tokio::test]
async fn auth() {
  setup_tracing();

  let api_url = std::env::var("2MCA_API_URL").unwrap_or_else(|_| "http://localhost:3000/platform2mca".to_string());
  let username = std::env::var("2MCA_API_USERNAME").unwrap_or_else(|_| "test".to_string());
  let password = std::env::var("2MCA_API_PASSWORD").unwrap_or_else(|_| "test".to_string());

  let client_user = whoami::username().unwrap_or_else(|_| "<unknown>".to_string());
  let client_name = whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string());
  let ip_address = local_ip_address::local_ip().map_or_else(|_| "<unknown>".to_owned(), |ip| ip.to_string());
  let mac_address = mac_address::get_mac_address()
    .ok()
    .flatten()
    .map_or_else(|| "<unknown>".to_owned(), |m| m.to_string());

  let client = Client::builder().base_url(api_url).build().unwrap();

  let res = client.authbasic(&Credentials { username, password }).await;
  assert!(res.is_ok());

  let session_id = res.unwrap();
  let hex32_regex = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
  assert!(
    hex32_regex.is_match(session_id.as_str()),
    "session_id '{}' is not a valid 32-char hex string",
    session_id.as_str()
  );

  let res = client.session_init(None).await;
  assert!(res.is_ok());

  let Session {
    debug_pipe_name,
    id: _id,
  } = res.unwrap();
  let pipe_regex = Regex::new(r"^debug\$\d{10}$").unwrap();
  assert!(
    pipe_regex.is_match(debug_pipe_name.as_str()),
    "debug_pipe_name '{}' does not match pattern 'debug$' + 10 digits",
    debug_pipe_name.as_str()
  );

  let res = client.system_core_info_get(&session_id).await;
  assert!(res.is_ok());

  let res = client.system_server_version_get(&session_id).await;
  assert!(res.is_ok());

  let res = client.system_settings_get(&session_id).await;
  assert!(res.is_ok());

  let res = client.protocol_info_get(&session_id).await;
  assert!(res.is_ok());

  let res = client.authentication_url_get().await;
  assert!(res.is_ok());

  let res = client.user_info_get(&session_id).await;
  assert!(res.is_ok());

  let res = client
    .system_net_address_set(&SystemNetAddressSet {
      session_id: session_id.clone(),
      mac_address,
      ip_address: ip_address.clone(),
    })
    .await;
  assert!(res.is_ok());

  let res = client.novo_allowed_check(&session_id).await;
  assert!(res.is_ok());

  let res = client.system_user_privileged_get(&session_id).await;
  assert!(res.is_ok());

  let res = client
    .network_information_set(&NetworkInformationSet {
      session_id: session_id.clone(),
      client_name,
      client_ip: ip_address,
      client_user,
      module_name: "ЦФТ - Навигатор 6.0.121.84".to_owned(),
    })
    .await;
  assert!(res.is_ok());

  let res = client
    .user_profile_property_get(&UserProfilePropertyGet {
      session_id: session_id.clone(),
      property_name: "SHOW_LOGINS_HISTORY".to_owned(),
    })
    .await;
  assert!(res.is_ok());

  let res = client
    .system_option_enabled_check(&SystemOptionEnabledCheck {
      session_id: session_id.clone(),
      option_name: "NAV_SKIN_INTERFACE".to_owned(),
    })
    .await;
  assert!(res.is_ok());

  let res = client
    .user_belongs_group_check(&UserBelongsGroupCheck {
      session_id: session_id.clone(),
      group_id: "DVS".to_owned(),
    })
    .await;
  assert!(res.is_ok());

  // let res = client.types_get(&session_id).await;
  // assert!(res.is_ok());

  // let res = client.guides_groups_get(&session_id).await;
  // assert!(res.is_ok());

  // let res = client.guides_get(&session_id).await;
  // assert!(res.is_ok());

  let res = client.user_menu_get(&session_id).await;
  assert!(res.is_ok());

  // let res = client
  //   .class_views_get(&ClassViewsGet {
  //     session_id: session_id.clone(),
  //     class_id: "USER".to_owned(),
  //   })
  //   .await;
  // assert!(res.is_ok());

  let res = client
    .class_children_get(&ClassChildrenGet {
      session_id: session_id.clone(),
      class_id: "USER".to_owned(),
    })
    .await;
  assert!(res.is_ok());

  let res = client
    .class_methods_groups_user_get(&ClassMethodsGroupsUserGet {
      session_id: session_id.clone(),
      class_id: "USER".to_owned(),
    })
    .await;
  assert!(res.is_ok());

  // let res = client
  //   .class_methods_get(&ClassMethodsGet {
  //     session_id: session_id.clone(),
  //     class_id: "USER".to_owned(),
  //   })
  //   .await;
  // assert!(res.is_ok());

  let res = client
    .class_need_collection_id_check(&ClassNeedCollectionIDCheck {
      session_id: session_id.clone(),
      class_id: "USER".to_owned(),
    })
    .await;
  assert!(res.is_ok());
  assert_eq!(res.unwrap(), CheckResult { value: "0".to_owned() });

  let res = client
    .view_columns_get(&ViewColumnsGet {
      session_id: session_id.clone(),
      view_id: "4384".to_owned(),
    })
    .await;
  assert!(res.is_ok());

  let res = client
    .class_states_get(&ClassStatesGet {
      session_id: session_id.clone(),
      class_id: "USER".to_owned(),
    })
    .await;
  assert!(res.is_ok());

  let res = client
    .class_transitions_get(&ClassTransitionsGet {
      session_id: session_id.clone(),
      class_id: "USER".to_owned(),
    })
    .await;
  assert!(res.is_ok());

  let res = client
    .view_data_get_cancelable(&ViewDataGetCancelable {
      session_id: session_id.clone(),
      view_short_name: "VW_CRIT_USER".to_string(),
      class_id: "USER".to_string(),
      hint: "FIRST_ROWS".to_string(),
      allow_timestamp_milliseconds: "true".to_string(),
      rows_limit: "10".to_string(),
    })
    .await;
  assert!(res.is_ok());

  let res = client
    .object_backward_references_get(&ObjectBackwardReferencesGet {
      session_id: session_id.clone(),
      object_id: "2350467263".to_string(),
      class_id: "USER".to_string(),
    })
    .await;
  assert!(res.is_ok());

  let res = client.session_deinit(&session_id).await;
  assert!(res.is_ok());
}
