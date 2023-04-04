use crate::models::user::User;
use mongodb::bson::Document;
use mongodb::{
    bson::{doc, Bson},
    options::ClientOptions,
    Client, Collection, Database,
};
use std::error::Error;

/// Database manager
///
/// # Fields
///
/// * `db` - Database
///
/// # Methods
///
/// * `new` - Create new database manager
/// * `insert_user` - Insert user to database
///

#[derive(Clone)]
pub struct DatabaseManager {
    db: Database,
}

impl DatabaseManager {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self, Box<dyn Error>> {
        let client_options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(db_name);

        Ok(Self { db })
    }

    pub async fn insert_user(&self, user: User) -> Result<(), Box<dyn Error>> {
        let collection = self.db.collection("user");

        let user_doc = doc! {
            "user_id": user.user_id,
            "username": user.username,
            "currency": Bson::Array(user.currency.into_iter().map(Bson::String).collect()),
            "created_at": user.created_at,
            "updated_at": user.updated_at,
        };

        collection.insert_one(user_doc, None).await?;
        // info!("Inserted user: {:?}", res.inserted_id);
        // info!("Inserted user: {:?}", res);
        Ok(())
    }

    pub async fn get_user(&self, user_id: i64) -> Result<User, Box<dyn Error>> {
        let collection: Collection<User> = self.db.collection("user");
        let user = collection.find_one(doc! {"user_id": user_id}, None).await?;
        Ok(user.unwrap())
    }

    // change user currency
    pub async fn change_user_currency(
        &self,
        user_id: i64,
        currency: String,
    ) -> Result<(), Box<dyn Error>> {
        let collection: Collection<User> = self.db.collection("user");
        let mut user = self.get_user(user_id).await?;
        user.currency.push(currency);
        let user_doc = self.update_user_doc(user).await;
        //collection.update_one(doc! {"user_id": user_id}, user_doc, None).await?;
        collection
            .update_one(doc! {"user_id": user_id}, doc! {"$set": user_doc}, None)
            .await?;
        Ok(())
    }

    pub async fn remove_user_currency(
        &self,
        user_id: i64,
        currency_to_remove: String,
    ) -> Result<(), Box<dyn Error>> {
        let collection: Collection<User> = self.db.collection("user");
        let mut user = self.get_user(user_id).await?;

        // Отфильтровать список валют, удаляя указанную валюту
        user.currency
            .retain(|currency| currency != &currency_to_remove);

        // Обновить документ с новым списком валют
        let user_doc = self.update_user_doc(user).await;

        collection
            .update_one(doc! {"user_id": user_id}, doc! {"$set": user_doc}, None)
            .await?;

        Ok(())
    }
    // update user document
    async fn update_user_doc(&self, user: User) -> Document {
        doc! {
            "user_id": user.user_id,
            "username": user.username,
            "currency": Bson::Array(user.currency.into_iter().map(Bson::String).collect()),
            "created_at": user.created_at,
            "updated_at": mongodb::bson::DateTime::now(),
        }
    }
}
