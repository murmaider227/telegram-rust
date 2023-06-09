use crate::models::user::User;
use futures::stream::StreamExt;
use log::debug;
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

        let user_doc = self.update_user_doc(user).await;

        let res = collection.insert_one(user_doc, None).await;
        // info!("Inserted user: {:?}", res.inserted_id);

        match res {
            Ok(_) => (),
            Err(err) => debug!("user already exists: {}", err),
        }

        Ok(())
    }

    pub async fn get_user(&self, user_id: i64) -> Option<User> {
        let collection: Collection<User> = self.db.collection("user");
        let user_query = collection.find_one(doc! {"user_id": user_id}, None).await;

        match user_query {
            Ok(Some(user)) => {
                // debug!("user Ok: {:?}", user);
                Some(user)
            }
            Ok(None) => {
                let usr = User::new(user_id, "".to_string(), vec![]);
                match usr.save(self.clone()).await {
                    Ok(_) => Some(usr),
                    Err(err) => {
                        debug!("user save error: {:?}", err);
                        None
                    }
                }
            }
            Err(err) => {
                debug!("get user error: {:?}", err);
                None
            }
        }
    }

    // change user currency
    pub async fn change_user_currency(
        &self,
        user_id: i64,
        currency: String,
    ) -> Result<(), Box<dyn Error>> {
        let collection: Collection<User> = self.db.collection("user");
        let user_query = self.get_user(user_id).await;
        let mut user = match user_query {
            Some(user) => user,
            None => return Err("Problem with user saving".into()),
        };
        user.currency.push(currency);
        let mut unique_currency: Vec<String> = Vec::new();
        for c in user.currency.into_iter() {
            if !unique_currency.contains(&c) {
                unique_currency.push(c);
            }
        }
        user.currency = unique_currency;
        let user_doc = self.update_user_doc(user).await;
        collection
            .update_one(doc! {"user_id": user_id}, doc! {"$set": user_doc}, None)
            .await?;
        //debug!("Updated user: {:?}", res.modified_count);
        Ok(())
    }

    pub async fn remove_user_currency(
        &self,
        user_id: i64,
        currency_to_remove: String,
    ) -> Result<(), Box<dyn Error>> {
        let collection: Collection<User> = self.db.collection("user");
        let user_query = self.get_user(user_id).await;

        let mut user = match user_query {
            Some(user) => user,
            None => return Err("Problem with user saving".into()),
        };

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
            "notification": user.notification,
        }
    }

    pub async fn get_all_users(
        &self,
        filter: Option<Document>,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let collection: Collection<User> = self.db.collection("user");
        let mut cursor = collection.find(filter, None).await?;
        let mut users_vec: Vec<User> = Vec::new();
        while let Some(result) = cursor.next().await {
            let user = result?;
            users_vec.push(user);
        }

        Ok(users_vec)
    }

    pub async fn change_notify(&self, mut user: User) -> Result<String, Box<dyn Error>> {
        let collection: Collection<User> = self.db.collection("user");
        user.notification = match user.notification {
            true => false,
            false => true,
        };
        let user_id = user.user_id;
        let user_doc = self.update_user_doc(user.clone()).await;
        let update_result = collection
            .update_one(doc! {"user_id": user_id}, doc! {"$set": user_doc}, None)
            .await?;

        if update_result.modified_count > 0 {
            let message = if user.notification {
                "successfully turned on"
            } else {
                "successfully turned off"
            };
            Ok(message.to_string())
        } else {
            Err("user not found".into())
        }
    }
}
