use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;

#[cfg_attr(test, derive(Serialize))]
#[derive(Deserialize)]
pub struct Repo {
    pub namespace: String,
    pub name: String,
    pub counter: u16,
}

#[cfg_attr(test, derive(Serialize))]
#[derive(Deserialize, Debug, PartialEq, Eq, Default)]
pub struct Build {
    pub number: u16,
    pub status: BuildStatus,
    pub message: String,
    #[serde(rename = "author_name")]
    pub author: String,
}

#[cfg_attr(test, derive(Serialize))]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_build_status() {
        assert_eq!(BuildStatus::default(), BuildStatus::Success);
    }
}
