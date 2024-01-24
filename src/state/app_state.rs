use mongodb::Client;

pub struct AppState {
    pub mongo_client: Client
}