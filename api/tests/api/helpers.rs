use api::startup::Application;

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    let application = Application::build([127, 0, 0, 1], 0).expect("Failed to build application.");
    let port = application.port();
    let _ = tokio::spawn(application.start_server());
    TestApp {
        address: format!("http://127.0.0.1:{port}"),
        port,
    }
}
