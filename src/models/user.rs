use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(rename="_id")]
    pub id: ObjectId,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterUser{
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser{
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
