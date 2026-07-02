pub mod common;

use as2mca_api::models::requests::{
  ClassChildrenGet, ClassInfo, ClassMethodsGet, ClassMethodsGroupsUserGet, ClassNeedCollectionIDCheck, ClassStatesGet,
  ClassTransitionsGet, ClassViewsGet, ClassesGet, DebugTextGet, MethodBegin, MethodControlsGet, MethodEnd,
  MethodParametersGet, MethodValidateDefault, MethodVariablesGet, NetworkInformationSet, Object,
  ObjectBackwardReferencesGet, ObjectClassAndArchiveKeyGet, ObjectFilter, ObjectsLock, ObjectsUnlock, PipeTextGet,
  SystemNetAddressSet, SystemOptionEnabledCheck, SystemSettingGet, UserBelongsGroupCheck, UserProfilePropertyGet,
  ViewColumnsGet, ViewDataGetCancelable,
};

use crate::common::setup;

#[tokio::test]
async fn test_system_and_user_info() {
  let (client, session_id, ..) = setup().await;

  let client_user = whoami::username().unwrap_or_else(|_| "<unknown>".to_string());
  let client_name = whoami::hostname().unwrap_or_else(|_| "<unknown>".to_string());
  let ip_address = local_ip_address::local_ip().map_or_else(|_| "<unknown>".to_owned(), |ip| ip.to_string());
  let mac_address = mac_address::get_mac_address()
    .ok()
    .flatten()
    .map_or_else(|| "<unknown>".to_owned(), |m| m.to_string());

  client.system_core_info_get(&session_id).await.unwrap();
  client.system_server_version_get(&session_id).await.unwrap();
  client.system_settings_get(&session_id).await.unwrap();
  client.protocol_info_get(&session_id).await.unwrap();
  client.authentication_url_get().await.unwrap();
  client.user_info_get(&session_id).await.unwrap();

  client
    .system_net_address_set(&SystemNetAddressSet {
      session_id: session_id.clone(),
      mac_address,
      ip_address: ip_address.clone(),
    })
    .await
    .unwrap();

  client.novo_allowed_check(&session_id).await.unwrap();
  client.system_user_privileged_get(&session_id).await.unwrap();

  client
    .network_information_set(&NetworkInformationSet {
      session_id: session_id.clone(),
      client_name,
      client_ip: ip_address,
      client_user,
      module_name: "ЦФТ - Навигатор 6.0.121.84".to_owned(),
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn test_get_options() {
  let (client, session_id, ..) = setup().await;

  client
    .user_profile_property_get(&UserProfilePropertyGet {
      session_id: session_id.clone(),
      property_name: "SHOW_LOGINS_HISTORY".to_owned(),
    })
    .await
    .unwrap();

  client
    .system_option_enabled_check(&SystemOptionEnabledCheck {
      session_id: session_id.clone(),
      option_name: "NAV_SKIN_INTERFACE".to_owned(),
    })
    .await
    .unwrap();

  client
    .user_belongs_group_check(&UserBelongsGroupCheck {
      session_id: session_id.clone(),
      group_id: "DVS".to_owned(),
    })
    .await
    .unwrap();

  client
    .system_setting_get(&SystemSettingGet {
      session_id: session_id.clone(),
      name: "NOVOMON.TIMEOUT_LIMIT".to_string(),
    })
    .await
    .unwrap();

  client.session_deinit(&session_id).await.unwrap();
}

#[tokio::test]
async fn test_types_guides_and_menu() {
  let (client, session_id, ..) = setup().await;

  client.types_get(&session_id).await.unwrap();
  client.guides_groups_get(&session_id).await.unwrap();
  client.guides_get(&session_id).await.unwrap();
  client.user_menu_get(&session_id).await.unwrap();

  client.session_deinit(&session_id).await.unwrap();
}

#[tokio::test]
async fn test_class_user_data() {
  let (client, session_id, ..) = setup().await;

  let class_id = "USER".to_owned();

  let views = client
    .class_views_get(&ClassViewsGet {
      session_id: session_id.clone(),
      class_id: class_id.clone(),
    })
    .await
    .unwrap();
  assert!(!views.is_empty());

  client
    .class_children_get(&ClassChildrenGet {
      session_id: session_id.clone(),
      class_id: class_id.clone(),
    })
    .await
    .unwrap();

  client
    .class_methods_groups_user_get(&ClassMethodsGroupsUserGet {
      session_id: session_id.clone(),
      class_id: class_id.clone(),
    })
    .await
    .unwrap();

  let methods = client
    .class_methods_get(&ClassMethodsGet {
      session_id: session_id.clone(),
      class_id: class_id.clone(),
    })
    .await
    .unwrap();
  assert!(!methods.is_empty());

  client
    .class_need_collection_id_check(&ClassNeedCollectionIDCheck {
      session_id: session_id.clone(),
      class_id: class_id.clone(),
    })
    .await
    .unwrap();

  client
    .class_states_get(&ClassStatesGet {
      session_id: session_id.clone(),
      class_id: class_id.clone(),
    })
    .await
    .unwrap();

  client
    .class_transitions_get(&ClassTransitionsGet {
      session_id: session_id.clone(),
      class_id: class_id.clone(),
    })
    .await
    .unwrap();

  client.session_deinit(&session_id).await.unwrap();
}

#[tokio::test]
async fn test_method() {
  let (client, session_id, ..) = setup().await;

  let method_id = 311;

  client
    .objects_lock(&ObjectsLock {
      session_id: session_id.clone(),
      objects: vec![Object {
        id: 22_738_256,
        class_id: "USER".to_string(),
      }],
    })
    .await
    .unwrap();

  let frame_id = client
    .method_begin(&MethodBegin {
      session_id: session_id.clone(),
      method_id,
    })
    .await
    .unwrap();

  let params = client
    .method_parameters_get(&MethodParametersGet {
      session_id: session_id.clone(),
      method_id,
    })
    .await
    .unwrap();
  assert!(!params.is_empty());

  let controls = client
    .method_controls_get(&MethodControlsGet {
      session_id: session_id.clone(),
      form_id: method_id,
    })
    .await
    .unwrap();
  assert!(!controls.is_empty());

  let variables = client
    .method_variables_get(&MethodVariablesGet {
      session_id: session_id.clone(),
      method_id: 2_544_731_869,
    })
    .await
    .unwrap();
  assert!(!variables.is_empty());

  let class_info = params
    .iter()
    .map(|p| ClassInfo {
      class_id: p.class_id.clone(),
    })
    .collect::<Vec<ClassInfo>>();
  assert!(!class_info.is_empty());

  let classes = client
    .classes_get(&ClassesGet {
      session_id: session_id.clone(),
      class_info,
    })
    .await
    .unwrap();
  assert!(!classes.is_empty());

  client
    .method_validate_default(&MethodValidateDefault {
      session_id: session_id.clone(),
      method_id,
      info: String::new(),
      do_commit: true,
      object_id: 22_738_256,
      class_id: "USER".to_string(),
      debug_level: 10,
      is_called_from_another_method: true,
      read_only: false,
      get_debug_text: true,
    })
    .await
    .unwrap();

  client
    .method_end(&MethodEnd {
      session_id: session_id.clone(),
      frame_id,
    })
    .await
    .unwrap();

  client
    .objects_unlock(&ObjectsUnlock {
      session_id: session_id.clone(),
      clear_all_locks: true,
    })
    .await
    .unwrap();

  client.session_deinit(&session_id).await.unwrap();
}

#[tokio::test]
async fn test_view_user_data() {
  let (client, session_id, ..) = setup().await;

  let class_id = "USER".to_string();
  let view_id = 4384;
  let view_short_name = "VW_CRIT_USER".to_string();
  let object_id = 22_738_256;

  let columns = client
    .view_columns_get(&ViewColumnsGet {
      session_id: session_id.clone(),
      view_id,
    })
    .await
    .unwrap();
  assert!(!columns.is_empty());

  let data = client
    .view_data_get_cancelable(&ViewDataGetCancelable {
      session_id: session_id.clone(),
      view_short_name,
      class_id: class_id.clone(),
      hint: "FIRST_ROWS".to_string(),
      allow_timestamp_milliseconds: true,
      rows_limit: None,
      body: Some(ObjectFilter { object_id }),
    })
    .await
    .unwrap();
  assert!(!data.is_empty());

  client
    .object_backward_references_get(&ObjectBackwardReferencesGet {
      session_id: session_id.clone(),
      object_id,
      class_id: class_id.clone(),
    })
    .await
    .unwrap();

  client
    .object_class_and_archive_key_get(&ObjectClassAndArchiveKeyGet {
      session_id: session_id.clone(),
      object_id,
      base_class_id: class_id.clone(),
    })
    .await
    .unwrap();

  client.session_deinit(&session_id).await.unwrap();
}

#[tokio::test]
async fn test_pipe_and_debug() {
  let (client, session_id, debug_pipe_name) = setup().await;

  client
    .pipe_text_get(&PipeTextGet {
      session_id: session_id.clone(),
      pipe_name: debug_pipe_name.clone(),
    })
    .await
    .unwrap();

  client
    .debug_text_get(&DebugTextGet {
      session_id: session_id.clone(),
      direction: "B".to_string(),
    })
    .await
    .unwrap();

  client.session_deinit(&session_id).await.unwrap();
}
