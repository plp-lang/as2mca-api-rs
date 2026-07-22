/// `cargo run --example view`
use as2mca_api::client::Client;
use as2mca_api::requests::ViewDataGetCancelable;
use as2mca_api::responses::Session;

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
