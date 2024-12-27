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
        let remote_url = env::var("SURREAL_REMOTE_URL").expect("cannot find variable"); // intentional panic
        let db_name = env::var("SURREAL_DB").expect("cannot find variable");
        let user = env::var("SURREAL_USER").expect("cannot find variable");
        let pass = env::var("SURREAL_PASSWORD").expect("cannot find variable");
        let namespace = env::var("SURREAL_NAMESPACE").expect("cannot find variable");

        let client = any::connect(remote_url).await?;

        client.use_ns(&namespace).use_db(&db_name).await?;

        // Authenticate
        client.signin(Root {
            username: &user,
            password: &pass,    
        }).await?;

        Ok(Self { client, namespace, db_name })
    }
}