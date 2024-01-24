use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginResponse{
    pub message: String,
    pub status: bool,
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Response{
    pub message: String,
    pub status: bool,
}