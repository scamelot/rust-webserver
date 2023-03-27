use mongodb::{options::ClientOptions, Client};
use dotenv::dotenv;
use std::env;


pub async fn connect_to_mongodb() -> Result<Client, mongodb::error::Error> {
    dotenv().ok();
    // Replace the connection string with your MongoDB server's connection string
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client_options = ClientOptions::parse(mongodb_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&env::var("MONGODB_DB").expect("MONGODB_DB must be set"));
    Ok(client)
}

//&env::var("MONGODB_COLLECTION").expect("MONGODB_COLLECTION must be set")