use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct User 
{
    pub login: String,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}