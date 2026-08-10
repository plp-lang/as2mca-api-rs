# История изменений

Формат основан на [Keep a Changelog](https://keepachangelog.com/ru/1.0.0/)

## [0.3.0] - 2026-08-10

### Добавлено

- Новые методы API:
  `system_info_get`, `system_limit_get`, `system_context_get`, `system_application_name_get`, `system_help_system_info_get`
  `embedded_interaction_available_check`, `embedded_interaction_required_check`, `embedded_interaction_get_resource`
  `context_information_available_check`, `embedded_interaction_post`, `embedded_interaction_get` [@Falldot].

### Изменено

- `system_info_get` и `system_context_get` теперь возвращают `Option<String>` вместо `String` (может отсутствовать) [@Falldot].
- `user_belongs_group_check`, `class_need_collection_id_check` теперь возвращают `String` [@Falldot].
- Расширены допустимые значения для типов `Invisible` и `Logging` [@Falldot].
- Поле `parent_id` структуры `Control` теперь десериализуется как `String` [@Falldot].
- Десериализация булевых значений теперь регистронезависима [@Falldot].

### Исправлено

- Неправильная XML сериализация булевых значений для структур запросов и ответов [@Falldot].

## [0.2.0] - 2026-08-03

### Добавлено

- Метод API `SystemContextInfoGet` [@Falldot].

### Изменено

- Переход с срезов на владеющие `Vec` для структур запросов [@Falldot].
- Изменены значения по умолчанию для некоторых запросов [@Falldot]:
  - `is_called_from_another_method` теперь `false` для `MethodValidateDefault`;
  - `get_debug_text` теперь `false` для `MethodValidateDefault` и `MethodValidate`;
  - `optimized_grid_updates` теперь `false` для `MethodValidate`.
- Обновлен пример из `./examples/fp_tune.rs` в соответствии с последними изменениями API [@Falldot].
- Обновлены примеры из `README` в соответствии с последними изменениями API [@Falldot].
- Расширен список версий модулей, с которыми тестировалась библиотека [@Falldot].

### Исправлено

- Неправильная XML сериализация для структур запросов [@Falldot].

## [0.1.1] - 2026-07-28

### Изменено

- Больше стандартных derive-макросов всем структурам туда, где это возможно [@Falldot].
