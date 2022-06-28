use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_returns_200() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/health_check", &app.address))
        .send()
        .await
        .expect("Failed to send request.");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.content_length(), Some(0));
}
