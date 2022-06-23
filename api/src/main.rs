use api::startup::Application;

// (仮) 設定をどこから取ってくるかは相談。
const HOST: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 8000;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let application = Application::build(HOST, PORT)?;
    application.start_server().await?;
    Ok(())
}
