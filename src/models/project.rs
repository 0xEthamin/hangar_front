use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project 
{
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub image_url: String,
    pub container_id: String,
    pub created_at: String,
    pub participants: Option<Vec<String>>,
}


#[derive(Deserialize)]
pub struct ProjectsResponse 
{
    pub projects: Vec<Project>,
}

#[derive(Deserialize)]
pub struct ProjectResponse 
{
    pub project: Project,
}

#[derive(Serialize)]
pub struct DeployPayload 
{
    pub project_name: String,
    pub image_url: String,
    pub participants: Vec<String>,
}