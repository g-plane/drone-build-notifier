#[macro_use]
extern crate log;

mod config;
mod notify;
mod request;
mod types;

use request::Client;
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = config::read_config_file().await?;

    let client = Client::new(&config.server, &config.token);
    let mut last_build = types::Build::default();

    let mut interval = time::interval(time::Duration::from_secs(10));
    loop {
        interval.tick().await;

        // fetch repo info again to gain latest build ID
        let repo = client.fetch_repo(&config.org, &config.repo).await?;
        let build = client.fetch_latest_build(&repo).await?;
        debug!("Build status of {} is: {:?}", build.number, build.status);

        if last_build == build {
            continue;
        } else {
            if build.status != types::BuildStatus::Running {
                notify::show_notification(&build)?;
            }
            last_build = build;
        }
    }
}
