use crate::types::*;
use reqwest::Result;

pub struct Client<'a> {
    server: &'a str,
    token: &'a str,
    client: reqwest::Client,
}

impl<'a> Client<'a> {
    pub fn new(server: &'a str, token: &'a str) -> Self {
        Self {
            server,
            token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_repo(&self, org: &str, repo: &str) -> Result<Repo> {
        self.client
            .get(format!("{}/api/repos/{}/{}", self.server, org, repo))
            .bearer_auth(self.token)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn fetch_latest_build(&self, repo: &Repo) -> Result<Build> {
        self.client
            .get(format!(
                "{}/api/repos/{}/{}/builds/{}",
                self.server, repo.namespace, repo.name, repo.counter
            ))
            .bearer_auth(self.token)
            .send()
            .await?
            .json()
            .await
    }
}
