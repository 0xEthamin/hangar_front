use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::database::DatabaseDetails;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProjectSourceType
{
    Direct,
    Github,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project
{
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub container_name: String,
    pub source: ProjectSourceType,
    pub source_url: String,
    pub source_branch: Option<String>,
    pub source_root_dir: Option<String>,
    pub deployed_image_tag: String,
    pub created_at: String,
    pub env_vars: Option<HashMap<String, String>>,
    pub persistent_volume_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ProjectDetails
{
    #[serde(flatten)]
    pub project: Project,
    pub participants: Vec<String>,
    pub database: Option<DatabaseDetails>,
}

#[derive(Deserialize)]
pub struct ProjectsResponse
{
    pub projects: Vec<Project>,
}

#[derive(Deserialize)]
pub struct ProjectDetailsResponse
{
    pub project: ProjectDetails,
}

#[derive(Serialize, Default)]
pub struct DeployPayload
{
    pub project_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_repo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_root_dir: Option<String>,
    pub participants: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_database: Option<bool>,
}

#[derive(Serialize)]
pub struct UpdateEnvPayload
{
    pub env_vars: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ProjectMetrics
{
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub memory_limit: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GlobalMetrics
{
    pub total_projects: i64,
    pub running_containers: u64,
    pub total_cpu_usage: f64,
    pub total_memory_usage_mb: f64,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DownProjectInfo
{
    #[serde(flatten)]
    pub project: Project,
    pub stopped_at: String,
    pub downtime_seconds: i64,
}

#[derive(Deserialize)]
pub struct DownProjectsResponse
{
    pub down_projects: Vec<DownProjectInfo>,
}