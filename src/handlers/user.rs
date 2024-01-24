use actix_web::{web, HttpResponse, Responder};
use crate::models::user::{RegisterUser, LoginUser};
use crate::models::jwt::Claims;
use crate::state::app_state::AppState;
use crate::services::user_service::UserService;
use bcrypt::{hash, DEFAULT_COST, verify};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use chrono;
use dotenv::dotenv;
use serde_json::json;
use actix_session::Session;

pub fn public_routes(cfg: &mut web::ServiceConfig){
    cfg.route("/register", web::post().to(register))
       .route("/login", web::post().to(login));
}

pub fn protected_user_routes(cfg: &mut web::ServiceConfig){
    cfg.route("/get_user", web::get().to(get_user));
}

fn create_jwt(id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("key must be set.");

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
}

fn validate_password(password: &str) -> bool { password.len() >= 8 }

fn hash_password(password: &str) -> Result<String, bcrypt:: BcryptError>{
    hash(password, DEFAULT_COST)
}

async fn register(data: web::Json<RegisterUser>, state: web::Data<AppState>) -> impl Responder {
    if !validate_password(&data.password){
        return HttpResponse::BadRequest().body("Password does not meet criteria.");
    } 
    
    let username = data.username.clone(); // Clone username before moving data
    let password = data.password.clone(); // Clone password for hashing

    match hash_password(&password) {
        Ok(hashed_password) => {
            let new_user = RegisterUser {
                username,
                password: hashed_password,
            };

            let user_service = UserService::new(state.mongo_client.clone());
            let username = new_user.username.clone();

            match user_service.add_user(new_user).await {
                Ok(_) => HttpResponse::Ok().body(format!("User {} Successfully Registered", username)),
                Err(e) => HttpResponse::InternalServerError().body(format!("Failed to register user: {}", e)),
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Failed to hash password"),
    }
}

async fn login(data: web::Json<LoginUser>, state: web::Data<AppState>, session: Session) -> impl Responder {
    let user_service = UserService::new(state.mongo_client.clone());

    match user_service.get_user_by_username(&data.username).await {
        Ok(Some(user)) =>{
            // Verify Password
            let user_id_str = user.id.to_hex();
            match verify(&data.password, &user.password){
                Ok(is_valid) if is_valid =>{
                    // Password is valid, create JWToken
                    match create_jwt(&user_id_str){
                        Ok(token) =>{
                            if let Err(_) = session.insert("user_id", &user_id_str) {
                                return HttpResponse::InternalServerError().body("Failed to set session.");
                            }

                            if let Err(_) = session.insert("auth-token", &token) {
                                return HttpResponse::InternalServerError().body("Failed to set session.");
                            }

                            HttpResponse::Ok().json(json!({"token": token}))
                        } 
                        Err(_) => HttpResponse::InternalServerError().body("Failed to generate token."),
                    }
                },
                _ => HttpResponse::Unauthorized().body("Invalid username or password."),
            }
        },
        Ok(None) => {
            HttpResponse::Unauthorized().body("Invalid username or password.")
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("An internal error occurred.")
        }
    }
}


async fn get_user(session: Session) -> impl Responder {
    if let Ok(Some(user_id)) = session.get::<String>("user_id") {
        // Use user_id for your logic
        HttpResponse::Ok().body(format!("User ID: {}", user_id))
    } else {
        HttpResponse::Unauthorized().body("User not logged in")
    }
}