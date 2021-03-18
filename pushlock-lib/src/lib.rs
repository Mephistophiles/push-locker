use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Locked {
    pub push_available: bool,
    pub locked_by: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
}
