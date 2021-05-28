use serde::Deserialize;

#[derive(Deserialize)]
pub struct Repo {
    pub namespace: String,
    pub name: String,
    pub counter: u16,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Default)]
pub struct Build {
    pub number: u16,
    pub status: BuildStatus,
    pub message: String,
    #[serde(rename = "author_name")]
    pub author: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BuildStatus {
    Running,
    Success,
    Failure,
}

impl Default for BuildStatus {
    fn default() -> Self {
        Self::Success
    }
}
