use std::sync::Once;

use rstest::fixture;

use as2mca_api::{client::Client, error::Error, responses::Session};

#[allow(dead_code)]
pub struct Context {
  pub client: Client,
  pub session_id: String,
  pub debug_pipe_name: String,
}

impl Context {
  #[allow(clippy::missing_errors_doc)]
  pub async fn with_client(client: Client, username: &str, password: &str) -> Result<Self, Error> {
    client.authbasic(username, password).await?;

    let Session {
      debug_pipe_name,
      session_id,
      ..
    } = client.session_init(None).await?;

    Ok(Self {
      client,
      session_id,
      debug_pipe_name,
    })
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    tokio::task::block_in_place(|| {
      tokio::runtime::Handle::current().block_on(async {
        let _ = self.client.session_deinit(&self.session_id).await;
      });
    });
  }
}

static INIT: Once = Once::new();

#[fixture]
pub async fn ctx() -> Context {
  INIT.call_once(|| {
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    let format = std::env::var("AS2MCA_API_LOG_FORMAT").map_or_else(|_| "pretty".to_string(), |s| s.to_lowercase());
    match format.as_str() {
      "pretty" => {
        tracing_subscriber::fmt().pretty().with_env_filter(filter).init();
      }
      "compact" => {
        tracing_subscriber::fmt().compact().with_env_filter(filter).init();
      }
      "json" => {
        tracing_subscriber::fmt().json().with_env_filter(filter).init();
      }
      _ => panic!("unknown logger format"),
    }
  });
  let api_url = std::env::var("AS2MCA_API_URL").unwrap_or_else(|_| "http://localhost:3000/platform2mca/".to_string());
  let username = std::env::var("AS2MCA_API_USERNAME").unwrap_or_else(|_| "test".to_string());
  let password = std::env::var("AS2MCA_API_PASSWORD").unwrap_or_else(|_| "test".to_string());

  let client = Client::new(api_url).expect("Failed to create client");

  Context::with_client(client, &username, &password)
    .await
    .expect("Failed to create context")
}
