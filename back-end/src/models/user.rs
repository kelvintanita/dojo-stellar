use serde::Serialize;
use utoipa::ToSchema; 

#[derive(Serialize, ToSchema)] 
pub struct User {
    pub id: u32,
    pub name: String,
}