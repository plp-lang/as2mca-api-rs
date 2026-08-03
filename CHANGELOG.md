# История изменений

Формат основан на [Keep a Changelog](https://keepachangelog.com/ru/1.0.0/)

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
