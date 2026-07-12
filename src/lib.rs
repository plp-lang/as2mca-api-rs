//! Библиотека для взаимодействия с API сервера приложений.
//!
//! Предоставляет типизированный клиент для вызова XML‑методов через HTTP.
//! Все запросы и ответы сериализуются/десериализуются с помощью `quick-xml` и `serde`.
//!
//! # Основные компоненты
//! - [`client::Client`] – основной клиент для выполнения запросов.
//! - [`requests`] – структуры, описывающие тела запросов.
//! - [`responses`] – структуры для разбора ответов сервера.
//! - [`error`] – типы ошибок, возникающих при работе.
//! - [`serde_helpers`] – вспомогательные утилиты для кастомизации сериализации.
//!
//! # Пример использования
//! ```no_run
//! use as2mca_api::client::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let client = Client::new("http://localhost:3000/platform2mca")?;
//!   client.authbasic("user", "pass").await?;
//!   let session = client.session_init(None).await?;
//!   let version = client.protocol_info_get(&session.session_id).await?;
//!
//!   println!("Protocol version: {}", version);
//!
//!   client.session_deinit(&session.session_id).await?;
//!
//!   Ok(())
//! }
//! ```
pub mod client;
pub mod error;
pub mod requests;
pub mod responses;
pub mod serde_helpers;
