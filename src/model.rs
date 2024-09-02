use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct  Post {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct  PostResponse {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}

