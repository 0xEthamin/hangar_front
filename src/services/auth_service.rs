use crate::models::user::User;
use gloo_net::http::Request;
use serde::Deserialize;

const API_ROOT: &str = "/api";

#[derive(Deserialize)]
struct AuthResponse 
{
    user: User,
}

#[derive(Deserialize)]
struct MeResponse 
{
    user: User,
}

pub async fn validate_ticket(ticket: &str) -> Result<User, String>
{
    let url = format!("{}/auth/callback?ticket={}", API_ROOT, ticket);
    
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() 
    {
        return Err(format!("API Error: status {}", response.status()));
    }

    response
        .json::<AuthResponse>()
        .await
        .map(|r| r.user)
        .map_err(|e| format!("Deserialization error: {}", e))
}


pub async fn get_current_user() -> Result<User, String> 
{
    Request::get(&format!("{}/auth/me", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?
        .json::<MeResponse>()
        .await
        .map(|r| r.user)
        .map_err(|_| "Not authenticated or session expired".to_string())
}


pub async fn logout() -> Result<(), String> 
{
    Request::get(&format!("{}/auth/logout", API_ROOT))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;
    Ok(())
}