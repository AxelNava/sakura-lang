mod client;

#[tokio::test]
async fn test_lsp_server() {
    client::_run_client().await;
}
