/// `cargo run --example fp_tune`
use as2mca_api::client::Client;
use as2mca_api::requests::{ControlState, MethodExecute, MethodValidate, MethodValidateDefault};
use as2mca_api::responses::{MethodResult, Session};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api_url = std::env::var("AS2MCA_API_URL").unwrap_or_else(|_| "http://localhost:3000/platform2mca/".to_string());
  let username = std::env::var("AS2MCA_API_USERNAME").unwrap_or_else(|_| "test".to_string());
  let password = std::env::var("AS2MCA_API_PASSWORD").unwrap_or_else(|_| "test".to_string());

  // Создаём клиент
  let client = Client::new(api_url)?;

  // Basic‑аутентификация и активация сессии
  client.authbasic(&username, &password).await?;
  let Session { ref session_id, .. } = client.session_init(None).await?;

  // Получаем список всех операций ТБП `::[FP_TUNE]`
  let class_short_name = "FP_TUNE";
  let methods = client.class_methods_get(session_id, class_short_name).await?;

  // Получаем `id` операции `::[FP_TUNE].[NEW#AUTO]`
  let method_short_name = "NEW#AUTO";
  let method_id = methods
    .iter()
    .find(|v| v.short_name == method_short_name)
    .expect("Операция не найдена!")
    .id;

  // Открываем форму и вызываем блок `validate` по умолчанию
  let frame_id = client.method_begin(session_id, method_id).await?;
  client
    .method_validate_default(&MethodValidateDefault {
      session_id,
      method_id,
      class_id: class_short_name,
      ..Default::default()
    })
    .await?;

  // Заполняем элемент формы "Группа", вызывая блок `validate` с соответсвующим `P_INFO`
  client
    .method_validate(&MethodValidate {
      session_id,
      method_id,
      info: "%PARAM%.P_GROUP_ID",
      controls_states: &[ControlState {
        id: 17_007_839,
        value: "AS2MCA_TEST_GROUP",
      }],
      ..Default::default()
    })
    .await?;

  // Заполняем элемент формы "Наименование", вызывая блок `validate` с соответсвующим `P_INFO`
  client
    .method_validate(&MethodValidate {
      session_id,
      method_id,
      info: "%PARAM%.P_NAME",
      controls_states: &[ControlState {
        id: 17_007_820,
        value: "Тестовая настройка",
      }],
      ..Default::default()
    })
    .await?;

  // Заполняем элемент формы "Код", вызывая блок `validate` с соответсвующим `P_INFO`
  client
    .method_validate(&MethodValidate {
      session_id,
      method_id,
      info: "%PARAM%.P_CODE",
      controls_states: &[ControlState {
        id: 17_007_818,
        value: "AS2MCA_TEST_CODE",
      }],
      ..Default::default()
    })
    .await?;

  // Выбираем тип значения как "Логика", вызывая блок `validate` с соответсвующим `P_INFO`
  client
    .method_validate(&MethodValidate {
      session_id,
      method_id,
      info: "%VAR%.V_VAL_TYPE.0",
      controls_states: &[ControlState {
        id: 17_007_844,
        value: "4",
      }],
      ..Default::default()
    })
    .await?;

  // Уставливаем значение, вызывая блок `validate` с соответсвующим `P_INFO`
  client
    .method_validate(&MethodValidate {
      session_id,
      method_id,
      info: "%VAR%.V_VAL_BOOL.0",
      controls_states: &[ControlState {
        id: 17_007_835,
        value: "1",
      }],
      ..Default::default()
    })
    .await?;

  // let MethodResult { value, .. } = client
  //   .method_execute(&MethodExecute {
  //     session_id,
  //     method_id,
  //     controls_states: &[
  //       ControlState {
  //         id: 17_007_839,
  //         value: "AS2MCA_TEST_GROUP",
  //       },
  //       ControlState {
  //         id: 17_007_820,
  //         value: "Тестовая настройка",
  //       },
  //       ControlState {
  //         id: 17_007_818,
  //         value: "AS2MCA_TEST_CODE",
  //       },
  //       ControlState {
  //         id: 17_007_864,
  //         value: "BOOLEAN",
  //       },
  //       ControlState {
  //         id: 17_007_835,
  //         value: "1",
  //       },
  //     ],
  //     ..Default::default()
  //   })
  //   .await?;
  // client.method_end(session_id, frame_id).await?;

  // client
  //   .method_validate(&MethodValidate {
  //     session_id,
  //     method_id,
  //     info: "%PLPCALL%",
  //     plpcall_parameters: &[
  //       PLPCallParameter {
  //         target: &[PLPEntity::PLPParameter(PLPParameter {
  //           method_id,
  //           name: "P_GROUP_ID",
  //         })],
  //         source: &[PLPEntity::PLPConstant(PLPConstant {
  //           value: "AS2MCA_TEST_GROUP",
  //         })],
  //       },
  //       PLPCallParameter {
  //         target: &[PLPEntity::PLPParameter(PLPParameter {
  //           method_id,
  //           name: "P_NAME",
  //         })],
  //         source: &[PLPEntity::PLPConstant(PLPConstant {
  //           value: "Тестовая настройка",
  //         })],
  //       },
  //       PLPCallParameter {
  //         target: &[PLPEntity::PLPParameter(PLPParameter {
  //           method_id,
  //           name: "P_CODE",
  //         })],
  //         source: &[PLPEntity::PLPConstant(PLPConstant {
  //           value: "AS2MCA_TEST_CODE",
  //         })],
  //       },
  //       PLPCallParameter {
  //         target: &[PLPEntity::PLPParameter(PLPParameter {
  //           method_id,
  //           name: "P_VAL_TYPE",
  //         })],
  //         source: &[PLPEntity::PLPConstant(PLPConstant { value: "BOOLEAN" })],
  //       },
  //       PLPCallParameter {
  //         target: &[PLPEntity::PLPParameter(PLPParameter {
  //           method_id,
  //           name: "P_VALUES",
  //         })],
  //         source: &[PLPEntity::PLPConstant(PLPConstant { value: "45543423508" })],
  //       },
  //     ],
  //     ..Default::default()
  //   })
  //   .await?;

  // Выполняем операцию, нажатие на кнопку "ОК" и закрытие формы
  let MethodResult { value, .. } = client
    .method_execute(&MethodExecute {
      session_id,
      method_id,
      ..Default::default()
    })
    .await?;
  client.method_end(session_id, frame_id).await?;

  // Печатаем в консоль `id` созданной настройки.
  println!("Была успешно добавлена новая настройка в `::[FP_TUNE]` с `id` = {value:?}");

  // Завершаем сессию
  client.session_deinit(session_id).await?;

  Ok(())
}
