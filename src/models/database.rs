use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct DatabaseDetails
{
    pub id: i32,
    pub database_name: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub project_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct DatabaseDetailsResponse
{
    pub database: DatabaseDetails,
}

#[derive(Deserialize)]
pub struct CreateDatabaseResponse
{
    pub database: DatabaseDetails,
}