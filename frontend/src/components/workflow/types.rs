use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[allow(non_camel_case_types)]
pub enum WorkflowRunConclusion {
    cancelled,
    failure,
    success,
}

impl Display for WorkflowRunConclusion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowRunConclusion::cancelled => write!(f, "cancelled"),
            WorkflowRunConclusion::failure => write!(f, "failure"),
            WorkflowRunConclusion::success => write!(f, "success"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowRun {
    pub id: i64,
    #[serde(rename = "workflow_id")]
    pub workflow_id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "head_branch")]
    pub head_branch: String,
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    #[serde(rename = "run_number")]
    pub run_number: i64,
    pub event: String,
    pub status: String,
    pub conclusion: WorkflowRunConclusion,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "jobs_url")]
    pub jobs_url: String,
    #[serde(rename = "logs_url")]
    pub logs_url: String,
    #[serde(rename = "check_suite_url")]
    pub check_suite_url: String,
    #[serde(rename = "artifacts_url")]
    pub artifacts_url: String,
    #[serde(rename = "cancel_url")]
    pub cancel_url: String,
    #[serde(rename = "rerun_url")]
    pub rerun_url: String,
    #[serde(rename = "workflow_url")]
    pub workflow_url: String,
    #[serde(rename = "head_commit")]
    pub head_commit: HeadCommit,
    pub repository: WorkflowRepository,
    #[serde(rename = "head_repository")]
    pub head_repository: HeadRepository,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadCommit {
    pub id: String,
    #[serde(rename = "tree_id")]
    pub tree_id: String,
    pub message: String,
    pub timestamp: String,
    pub author: Author,
    pub committer: Committer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Committer {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowRepository {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadRepository {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: Owner2,
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner2 {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}
