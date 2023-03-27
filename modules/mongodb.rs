use mongodb::{options::ClientOptions, Client};

pub async fn connect_to_mongodb() -> Result<Client, mongodb::error::Error> {
    // Replace the connection string with your MongoDB server's connection string
    let mongodb_uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(mongodb_uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}
