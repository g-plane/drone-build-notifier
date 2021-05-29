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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_fetch_repo() {
        let fake_token = "my-token";

        let mock = mock("GET", "/api/repos/my-org/my-repo")
            .with_header("Content-Type", "application/json")
            .with_body(
                serde_json::to_string(&Repo {
                    namespace: String::from("my-org"),
                    name: String::from("my-repo"),
                    counter: 2,
                })
                .unwrap(),
            )
            .match_header("Authorization", &*format!("Bearer {}", fake_token))
            .create();

        let server = server_url();
        let client = Client::new(&server, fake_token);
        let repo = client.fetch_repo("my-org", "my-repo").await.unwrap();

        assert_eq!(&*repo.namespace, "my-org");
        assert_eq!(&*repo.name, "my-repo");
        assert_eq!(repo.counter, 2);
        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_latest_build() {
        let fake_token = "my-token";
        let repo = Repo {
            namespace: String::from("my-org"),
            name: String::from("my-repo"),
            counter: 5,
        };

        let mock = mock("GET", "/api/repos/my-org/my-repo/builds/5")
            .with_header("Content-Type", "application/json")
            .with_body(
                serde_json::to_string(&Build {
                    number: repo.counter,
                    status: BuildStatus::Success,
                    message: String::from("first commit"),
                    author: String::from("Pig"),
                })
                .unwrap(),
            )
            .match_header("Authorization", &*format!("Bearer {}", fake_token))
            .create();

        let server = server_url();
        let client = Client::new(&server, fake_token);
        let build = client.fetch_latest_build(&repo).await.unwrap();

        assert_eq!(build.number, repo.counter);
        assert_eq!(build.status, BuildStatus::Success);
        assert_eq!(&*build.message, "first commit");
        assert_eq!(&*build.author, "Pig");
        mock.assert();
    }
}
