use crate::models::project::{DeployPayload, Project, ProjectResponse, ProjectsResponse};
use gloo_net::http::Request;
use serde::Deserialize;

const API_ROOT: &str = "/api";

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct ApiError 
{
    pub error_code: String,
    pub details: Option<String>,
}

#[derive(Deserialize)]
pub struct StatusResponse 
{
    pub status: Option<String>,
}

/// Tente de parser un code d'erreur structuré depuis une réponse HTTP qui n'est pas "ok".
/// Si le parsing échoue, retourne une erreur basée sur le statut HTTP.
async fn parse_simple_error_response(response: gloo_net::http::Response) -> String 
{
    #[derive(Deserialize)]
    struct SimpleErrorResponse { error_code: String }
    
    if let Ok(error_body) = response.json::<SimpleErrorResponse>().await 
    {
        error_body.error_code
    } 
    else 
    {
        format!("HTTP_ERROR_{}", response.status())
    }
}

pub async fn parse_detailed_error_response(response: gloo_net::http::Response) -> ApiError 
{
    if let Ok(error_body) = response.json::<ApiError>().await 
    {
        error_body
    } 
    else 
    {
        ApiError 
        {
            error_code: format!("HTTP_ERROR_{}", response.status()),
            details: None,
        }
    }
}


pub async fn get_owned_projects() -> Result<Vec<Project>, String>
{
    let response = Request::get(&format!("{}/projects/owned", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    { 
        return Err(parse_simple_error_response(response).await); 
    }

    response
        .json::<ProjectsResponse>()
        .await
        .map(|r| r.projects)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn get_participating_projects() -> Result<Vec<Project>, String>
{
    let response = Request::get(&format!("{}/projects/participations", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok()
    {
        return Err(parse_simple_error_response(response).await); 
    }

    response
        .json::<ProjectsResponse>()
        .await
        .map(|r| r.projects)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn deploy_project(
    project_name: &str,
    image_url: &str,
    participants: Vec<String>,
) -> Result<Project, ApiError>
{
    let payload = DeployPayload
    {
        project_name: project_name.to_string(),
        image_url: image_url.to_string(),
        participants,
    };

    let response = Request::post(&format!("{}/projects/deploy", API_ROOT))
        .json(&payload)
        .map_err(|_| ApiError 
        { 
            error_code: "CLIENT_SERIALIZATION_ERROR".to_string(),
            details: None,
        })?
        .send()
        .await
        .map_err(|e| ApiError 
        {
            error_code: "NETWORK_ERROR".to_string(),
            details: Some(e.to_string()),
        })?;


    if !response.ok() 
    {
        return Err(parse_detailed_error_response(response).await);
    }

    response
        .json::<ProjectResponse>()
        .await
        .map(|pr| pr.project)
        .map_err(|e| ApiError
        {
            error_code: "RESPONSE_PARSE_ERROR".to_string(),
            details: Some(e.to_string()),
        })
}

pub async fn purge_project(project_id: i32) -> Result<(), String>
{
    let response = Request::delete(&format!("{}/projects/{}", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok()
    {
        return Err(parse_simple_error_response(response).await); 
    }

    Ok(())
}

pub async fn get_project_details(project_id: i32) -> Result<Project, String>
{
    let response = Request::get(&format!("{}/projects/{}", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok()
    {
        return Err(parse_simple_error_response(response).await); 
    }

    response
        .json::<ProjectResponse>()
        .await
        .map(|r| r.project)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn get_project_status(project_id: i32) -> Result<Option<String>, String> 
{
    let response = Request::get(&format!("{}/projects/{}/status", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }

    response
        .json::<StatusResponse>()
        .await
        .map(|r| r.status)
        .map_err(|e| format!("Failed to parse response: {}", e))
}

pub async fn start_project(project_id: i32) -> Result<(), String> 
{
    let response = Request::post(&format!("{}/projects/{}/start", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }
    Ok(())
}

pub async fn stop_project(project_id: i32) -> Result<(), String> 
{
    let response = Request::post(&format!("{}/projects/{}/stop", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }
    Ok(())
}

pub async fn restart_project(project_id: i32) -> Result<(), String> 
{
    let response = Request::post(&format!("{}/projects/{}/restart", API_ROOT, project_id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(parse_simple_error_response(response).await);
    }
    Ok(())
}