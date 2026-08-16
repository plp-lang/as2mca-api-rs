# HTTP-клиент сервера приложений 2 MCA АБС ЦФТ

<h4 align="center">
  <a href="https://crates.io/crates/as2mca-api">
    <img src="https://img.shields.io/crates/v/as2mca-api?color=orange&logo=hack-the-box&logoColor=ea7233" alt="Rust Crate" />
  </a>
  <a href="https://github.com/plp-lang/as2mca-api-rs">
    <img src="https://img.shields.io/badge/GitHub-repo-dea584?logo=rust" alt="Rust Source" />
  </a>
  <a href="https://www.npmjs.com/package/as2mca-api">
      <img src="https://img.shields.io/npm/v/as2mca-api?color=red&logo=npm" alt="npm" />
  </a>
  <a href="https://github.com/plp-lang/as2mca-api-ts">
      <img src="https://img.shields.io/badge/GitHub-repo-3178C6?logo=typescript" alt="TypeScript Source" />
  </a>
  <a href="https://github.com/plp-lang/as2mca-api-rs/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT license" />
  </a>
</h4>

Неофициальная, свободная асинхронная Rust-библиотека, предоставляющая типизированный клиент для взаимодействия с API сервера приложений Платформы 2 MCA АБС ЦФТ в том числе для режима эмуляции Платформы 1.
Библиотека позволяет обращаться к серверу приложений аналогично клиенту «ЦФТ - Навигатор», программно реализуя схожее поведение: от управления сессиями и аутентификации до работы с выборками представлений и вызова операций.

Проект создан исключительно в некоммерческих, образовательных и исследовательских целях. Основные направления применения:

- **Изучение API** сервера приложений Платформы 2 MCA (понимание протокола, форматов XML и жизненного цикла сессий);
- **Интеграционное и автоматизированное тестирование** бизнес-логики, результатов выборок представлений и исполнения операций в АБС;
- **Интеграция со сторонними сервисами** (разработка бэкендов, микросервисов, шлюзов и скриптов автоматизации);
- **Разработка альтернативных клиентских приложений** (создание собственных GUI-интерфейсов, веб-клиентов или мобильных приложений) для работы с банковской системой.

> [!WARNING]
> Данный проект является **неофициальным** и **не аффилирован** с компанией «ЦФТ» или разработчиками Платформы 2 MCA.
> Библиотека создана на основе самостоятельного анализа поведения клиента «ЦФТ - Навигатор» и открытых сетевых обменов.
>
> Проект не гарантирует _полную_ совместимость с сервером приложений. В зависимости от:
>
> - Версии протокола обмена,
> - Конфигурации сервера, установленных патчей и обновлений ТЯ,
> - Политик безопасности и сетевых настроек конкретного контура.
>
> Структура запросов/ответов API и жизненный цикл сессий могут отличаться.
>
> Использование этого проекта в продуктовых контурах банков или в нарушение лицензионных соглашений и политик безопасности «ЦФТ» осуществляется исключительно на ваш страх и риск.
> Автор не несет ответственности за любые прямые или косвенные последствия использования этой библиотеки.

## Полезные ссылки

- Библиотека на [crates.io](https://crates.io/crates/as2mca-api)
- Альтернативная реализация на **Typescript**: [npm](https://www.npmjs.com/package/as2mca-api) | [source](https://github.com/plp-lang/as2mca-api-ts)

## Основные возможности

- **Управление сессией**: Basic‑аутентификация, активация/деактивация сессии;
- **Системная информация**: версия протокола, версия БД, настройки системы;
- **Информация о пользователе**: параметры, группы, привилегии;
- **Работа с ТБП и типами**: получение списка справочников, типов и переходов состояний;
- **Операции**: открытие формы, получение параметров, переменных, элементов формы, вызов блоков `Validate` и `Execute`;
- **Представления**: получение данных, колонок, списка представлений для ТБП;
- **Блокировки**: блокировка/разблокировка экземпляров.

Все запросы и ответы строго типизированы, используют [serde](https://github.com/serde-rs/serde) и [quick-xml](https://github.com/tafia/quick-xml) для работы с XML.

## Совместимость

Библиотека тестировалась с следующими версиями:

- **Protocol** (`protocolInfoGet`): `9.54`;
- **ТЯ**: `7.6.5.0`, `7.7.4.9`;
- **АБС**: `26.2.03-26.2.14`, `26.3.09`;
- **СП DBI Oracle**: `3.11.115-3.11.128`;
- **СП DBI Oracle в режиме эмуляции Платформы 1**: `2.67.9`;
- **Java-компилятор**: `7.9.6-7.9.16`.

Если ваш сервер использует другие версии, некоторые структуры могут не совпадать.
В таком случае прошу открыть Issue или Pull Request.

## Пример использования

Этот пример использует `Tokio` c дополнительными функциями. Ваш `Cargo.toml` может выглядеть так:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
as2mca_api = "0.3"
```

### Пример 1

Получение данных из представления `::[USER].[VW_CRIT_USER]`:

```rs
use as2mca_api::client::Client;
use as2mca_api::requests::ViewDataGetCancelable;
use as2mca_api::responses::Session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Создаём клиент
  let client = Client::new(api_url)?;

  // Basic‑аутентификация и активация сессии
  client.authbasic(&username, &password).await?;
  let Session { ref session_id, .. } = client.session_init(None).await?;

  // Получаем 5 первых строк представления `::[USER].[VW_CRIT_USER]`
  let rows = client
    .view_data_get_cancelable(&ViewDataGetCancelable {
      session_id,
      view_short_name: "VW_CRIT_USER",
      class_id: "USER",
      rows_limit: Some(5),
      ..Default::default()
    })
    .await?;

  // Печатаем данные в консоль
  for row in rows {
    for item in row.row_item {
      println!("{} = {}", item.column_name, item.value);
    }
    println!("---");
  }

  // Завершаем сессию
  client.session_deinit(session_id).await?;

  Ok(())
}
```

### Пример 2

Добавить свою настройку в `::[FP_TUNE]`, через операцию `::[FP_TUNE].[NEW#AUTO]`

Авторизация и инициализация сессии:

```rs
use as2mca_api::client::Client;
use as2mca_api::requests::{ControlState, MethodExecute, MethodValidate, MethodValidateDefault};
use as2mca_api::responses::{MethodResult, Session};

let client = Client::new(api_url)?;

client.authbasic(&username, &password).await?;
let Session { ref session_id, .. } = client.session_init(None).await?;
```

Получаем список всех операций ТБП `::[FP_TUNE]` и находим `ID` операции `[NEW#AUTO]` для последующих запросов:

```rs
let class_short_name = "FP_TUNE";
let method_short_name = "NEW#AUTO";

let methods = client.class_methods_get(session_id, class_short_name).await?;
let method_id = methods
  .iter()
  .find(|v| v.short_name == method_short_name)
  .expect("Операция не найдена!")
  .id;
```

Открываем форму и получаем `frameId`, который понадобится для закрытия формы:

```rs
let frame_id = client.method_begin(session_id, method_id).await?;
```

Опционально вызываем блок `VALIDATE` по умолчанию, `P_MESSAGE = 'DEFAULT'`:

```rs
client
  .method_validate_default(&MethodValidateDefault {
    session_id,
    method_id,
    class_id: class_short_name,
    ..Default::default()
  })
  .await?;
```

#### Дальше у нас несколько варантов исполнения операции

**Вариант 1**: поочередно заполнять элементы формы и вызывать блок `VALIDATE` как событие `P_MESSAGE = 'VALIDATE'` с соответствующим `P_INFO`:

```rs
// Заполняем элемент формы "Группа"
client
  .method_validate(&MethodValidate {
    session_id,
    method_id,
    info: "%PARAM%.P_GROUP_ID",
    controls_states: vec![ControlState {
      id: 17_007_839,
      value: "AS2MCA_TEST_GROUP",
    }],
    ..Default::default()
  })
  .await?;

// Заполняем элемент формы "Наименование"
client
  .method_validate(&MethodValidate {
    session_id,
    method_id,
    info: "%PARAM%.P_NAME",
    controls_states: vec![ControlState {
      id: 17_007_820,
      value: "Тестовая настройка",
    }],
    ..Default::default()
  })
  .await?;

// Заполняем элемент формы "Код"
client
  .method_validate(&MethodValidate {
    session_id,
    method_id,
    info: "%PARAM%.P_CODE",
    controls_states: vec![ControlState {
      id: 17_007_818,
      value: "AS2MCA_TEST_CODE",
    }],
    ..Default::default()
  })
  .await?;

// Выбираем тип значения как "Логика"
client
  .method_validate(&MethodValidate {
    session_id,
    method_id,
    info: "%VAR%.V_VAL_TYPE.0",
    controls_states: vec![ControlState {
      id: 17_007_844,
      value: "4",
    }],
    ..Default::default()
  })
  .await?;

// Уставливаем значение
client
  .method_validate(&MethodValidate {
    session_id,
    method_id,
    info: "%VAR%.V_VAL_BOOL.0",
    controls_states: vec![ControlState {
      id: 17_007_835,
      value: "1",
    }],
    ..Default::default()
  })
  .await?;

// Выполняем операцию, нажатие на кнопку "ОК" и закрытие формы
let MethodResult { value, .. } = client
  .method_execute(&MethodExecute {
    session_id,
    method_id,
    ..Default::default()
  })
  .await?;
println!("Была успешно добавлена новая настройка в `::[FP_TUNE]` с `id` = {value:?}");
```

**Вариант 2**: вызвать блок `EXECUTE` операции, с заранее заполненными элементами формы:

```rs
let MethodResult { value, .. } = client
  .method_execute(&MethodExecute {
    session_id,
    method_id,
    controls_states: vec![
      ControlState {
        id: 17_007_839,
        value: "AS2MCA_TEST_GROUP",
      },
      ControlState {
        id: 17_007_820,
        value: "Тестовая настройка",
      },
      ControlState {
        id: 17_007_818,
        value: "AS2MCA_TEST_CODE",
      },
      ControlState {
        id: 17_007_864,
        value: "BOOLEAN",
      },
      ControlState {
        id: 17_007_835,
        value: "1",
      },
    ],
    ..Default::default()
  })
  .await?;
println!("Была успешно добавлена новая настройка в `::[FP_TUNE]` с `id` = {value:?}");
```

**Вариант 3**: вызвать блок `VALIDATE` с параметрами, как `PLPCALL`

Аналогично команде: `<%PLPCALL [FP_TUNE].[TEST2](%PARAM%.P_CODE => 'AS2MCA_TEST_CODE', %PARAM%.P_NAME => 'Тестовая настройка'", и т.д...) %>`

```rs
client
  .method_validate(&MethodValidate {
    session_id,
    method_id,
    info: "%PLPCALL%",
    plpcall_parameters: vec![
      PLPCallParameter {
        target: vec![PLPEntity::PLPParameter(PLPParameter {
          method_id,
          name: "P_GROUP_ID",
        })],
        source: vec![PLPEntity::PLPConstant(PLPConstant {
          value: "AS2MCA_TEST_GROUP",
        })],
      },
      PLPCallParameter {
        target: vec![PLPEntity::PLPParameter(PLPParameter {
          method_id,
          name: "P_NAME",
        })],
        source: vec![PLPEntity::PLPConstant(PLPConstant {
          value: "Тестовая настройка",
        })],
      },
      PLPCallParameter {
        target: vec![PLPEntity::PLPParameter(PLPParameter {
          method_id,
          name: "P_CODE",
        })],
        source: vec![PLPEntity::PLPConstant(PLPConstant {
          value: "AS2MCA_TEST_CODE",
        })],
      },
      PLPCallParameter {
        target: vec![PLPEntity::PLPParameter(PLPParameter {
          method_id,
          name: "P_VAL_TYPE",
        })],
        source: vec![PLPEntity::PLPConstant(PLPConstant { value: "BOOLEAN" })],
      },
      PLPCallParameter {
        target: vec![PLPEntity::PLPParameter(PLPParameter {
          method_id,
          name: "P_VALUES",
        })],
        source: vec![PLPEntity::PLPConstant(PLPConstant { value: "45543423508" })],
      },
    ],
    ..Default::default()
  })
  .await?;

// Выполняем операцию, нажатие на кнопку "ОК"
let MethodResult { value, .. } = client
  .method_execute(&MethodExecute {
    session_id,
    method_id,
    ..Default::default()
  })
  .await?;
println!("Была успешно добавлена новая настройка в `::[FP_TUNE]` с `id` = {value:?}");
```

Закрываем форму операции, передав `frameId`, полученный из метода `methodBegin`:

```ts
client.method_end(session_id, frame_id).await?;
```

Не забываем закрыть сессию:

```ts
client.session_deinit(session_id).await?;
```

> Больше примеров можно посмотреть в [examples](https://github.com/plp-lang/as2mca-api-rs/tree/main/examples) или в [tests](https://github.com/plp-lang/as2mca-api-rs/tree/main/tests).

## Лицензия

[MIT](https://github.com/plp-lang/as2mca-api-ts/blob/master/LICENSE)
