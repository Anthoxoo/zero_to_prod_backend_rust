#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // iterator that goes read in the config file
    let settings = config::Config::builder()
        // expliciting name for the configuration file that will be configuration.yaml.
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;

    // puts everything in our settings struct
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn settings_to_string(&self) -> String {
        // While we deserialize to get everything in the struct, postgres wants a url like link to connect.
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}
