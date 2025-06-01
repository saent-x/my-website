use surrealdb::{engine::any::{self, Any}, opt::auth::Root, Error, Surreal };
use std::env;

#[derive(Clone)]
pub struct Database{
    pub client: Surreal<Any>,
    pub namespace: String,
    pub db_name: String,
}

impl Database {
    pub async fn init() -> Result<Self, Error>{
        // load env variables
        let remote_url = env!("SURREAL_REMOTE_URL");
        let db_name = env!("SURREAL_DB");
        let user = env!("SURREAL_USER");
        let pass = env!("SURREAL_PASSWORD");
        let namespace = env!("SURREAL_NAMESPACE");
        
        let client = any::connect(remote_url).await?;

        client.use_ns(namespace).use_db(db_name).await?;

        // Authenticate
        client.signin(Root {
            username: &user,
            password: &pass,    
        }).await?;

        Ok(Self { client, namespace: namespace.to_string(), db_name: db_name.to_string() })
    }
}