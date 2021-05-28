use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize)]
pub struct Config {
    pub server: String,
    pub token: String,
    pub org: String,
    pub repo: String,
}

pub async fn read_config_file() -> anyhow::Result<Config> {
    let config_file = fs::read_to_string("config.toml").await?;

    toml::from_str::<Config>(&config_file).map_err(anyhow::Error::from)
}
