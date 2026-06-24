use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubApiOwner {
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubApiRepository {
    pub id: i64,
    pub full_name: String,
    pub owner: GitHubApiOwner,
    pub description: Option<String>,
    pub default_branch: Option<String>,
    pub html_url: Option<String>,
    pub private: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubApiUser {
    pub id: i64,
    pub login: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GitHubApiPullRequestMarker {
    #[allow(dead_code)]
    url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubApiIssue {
    pub id: i64,
    pub number: i64,
    pub title: String,
    pub state: String,
    pub html_url: Option<String>,
    pull_request: Option<GitHubApiPullRequestMarker>,
}

impl GitHubApiIssue {
    pub fn is_pull_request(&self) -> bool {
        self.pull_request.is_some()
    }
}
