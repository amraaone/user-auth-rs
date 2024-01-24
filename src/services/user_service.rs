use mongodb::{Client, Collection};
use crate::models::user::{RegisterUser, User};
use mongodb::bson::{doc};

pub struct UserService {
    client: Client,
}

impl UserService {
    pub fn new(client: Client) -> Self {
        UserService { client }
    }

    pub async fn add_user(&self, user_data: RegisterUser) -> mongodb::error::Result<()> {
        let collection: Collection<RegisterUser> = self.client
            .database("rusty")
            .collection("users");
        
        collection.insert_one(user_data, None).await?;
        Ok(())
    }

    pub async fn get_user_by_username(&self, username: &str) -> mongodb::error::Result<Option<User>>{
        let collection: Collection<User> = self.client
            .database("rusty")
            .collection("users");
        
        let filter = doc! {"username": username};

        match collection.find_one(filter, None).await {
            Ok(Some(user)) => {
                Ok(Some(user))
            },
            Ok(None) => {
                Ok(None)
            },
            Err(e) => {
                Err(e)
            }
        }

    }
}
