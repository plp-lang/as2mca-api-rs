use rstest::rstest;

use crate::common::ctx::{Context, ctx};

mod common;

#[rstest]
#[case("AC_FIN")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_methods(#[future] ctx: Context, #[case] class_short_name: &str) {
  let Context {
    ref client,
    ref session_id,
    ..
  } = ctx.await;

  let methods = client.class_methods_get(session_id, class_short_name).await.unwrap();
  for method in methods {
    let _script = client.method_client_script_get(session_id, method.id).await.unwrap();
    let _controls = client
      .method_controls_get(session_id, method.form_id.unwrap_or(method.id))
      .await
      .unwrap();
    let _params = client.method_parameters_get(session_id, method.id).await.unwrap();
    let _vars = client.method_variables_get(session_id, method.id).await.unwrap();
  }
}
