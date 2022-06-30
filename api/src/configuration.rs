use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::{deserialize_bool_from_anything, deserialize_number_from_string};

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub database_name: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn with_db(&self) -> String {
        let ssl_mode: PgSslMode = self.require_ssl.into();
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode={}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name,
            ssl_mode.as_str()
        )
    }

    pub fn without_db(&self) -> String {
        let ssl_mode: PgSslMode = self.require_ssl.into();
        format!(
            "postgresql://{}:{}@{}:{}/postgres?sslmode={}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            ssl_mode.as_str()
        )
    }
}

pub enum PgSslMode {
    Require,
    Prefer,
}

impl PgSslMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            PgSslMode::Require => "require",
            PgSslMode::Prefer => "prefer",
        }
    }
}

impl From<bool> for PgSslMode {
    fn from(require_ssl: bool) -> Self {
        match require_ssl {
            true => PgSslMode::Require,
            false => PgSslMode::Prefer,
        }
    }
}

pub enum AppEnvironment {
    Local,
    Production,
}

impl AppEnvironment {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppEnvironment::Local => "local",
            AppEnvironment::Production => "production",
        }
    }
}

impl TryInto<AppEnvironment> for String {
    type Error = String;

    fn try_into(self) -> Result<AppEnvironment, Self::Error> {
        match self.to_lowercase().as_str() {
            "local" => Ok(AppEnvironment::Local),
            "production" => Ok(AppEnvironment::Production),
            other => Err(format!(
                "`{other}` is not supported environment. Use either `local` or `production`."
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let configuration_directory = base_path.join("configuration");

    let app_environment: AppEnvironment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse `APP_ENVIRONMENT`.");

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("base")).required(true))
        .add_source(
            config::File::from(configuration_directory.join(app_environment.as_str()))
                .required(true),
        )
        // E.g. `APP_APPLICATION__HOST` は `application.host` にセットされる
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize()
}
