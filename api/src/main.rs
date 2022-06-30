use api::configuration::get_configuration;
use api::startup::Application;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration)?;
    application.start_server().await?;
    Ok(())
}
