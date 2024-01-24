mod handlers;
mod models;
mod state;
mod services;
mod middleware;

use dotenv::dotenv;
use std::env;
use actix_web::{web, App, HttpServer};
use handlers::user::{protected_user_routes, public_routes};
use mongodb::{Client, options::ClientOptions};
use state::app_state::AppState;
use middleware::auth::JwtMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_web::middleware::Condition;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Set Up address
    let backend_port = env::var("BACKEND_PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("127.0.0.1:{}", backend_port);

    // Set Up MongoDB Connection
    let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017/rusty".to_string());
    let mongo_client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    let mongo_client = Client::with_options(mongo_client_options).unwrap();

    HttpServer::new(move|| {
        App::new()
            .configure(public_routes)
            .service(web::scope("/user").wrap(Condition::new(true, JwtMiddleware)).configure(protected_user_routes))
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64])).build())
            .app_data(web::Data::new(AppState{mongo_client: mongo_client.clone()}))
    })
    .bind(server_address)? 
    .run()
    .await
}
