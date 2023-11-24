use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub internal_flag: bool,
    pub create_timestamp: DateTime<Utc>,
    pub create_user: String,
    pub update_timestamp: DateTime<Utc>,
}
