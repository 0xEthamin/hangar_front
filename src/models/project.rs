use serde::{Deserialize, Serialize};

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
    pub deployed_image_tag: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDetails 
{
    #[serde(flatten)]
    pub project: Project,
    pub participants: Vec<String>,
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
    pub participants: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ProjectMetrics 
{
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub memory_limit: f64,
}