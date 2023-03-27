use actix_web::{get,web, App, HttpServer, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;

// Import the newly created modules
mod modules {
    pub mod mongodb;
    pub mod api;
}

// Use the modules in the main.rs
use modules::{api, mongodb};

// Define an async function to serve the index.html file
#[get("/")]
async fn index() -> impl Responder {
    let path: PathBuf = PathBuf::from("./static/index.html");
    NamedFile::open(path).unwrap()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Connect to MongoDB
    let mongodb_client = match mongodb::connect_to_mongodb().await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error connecting to MongoDB: {}", e);
            std::process::exit(1);
        }
    };

    println!("Connected to MongoDB!");

    // Set the server's address
    let addr = "127.0.0.1:8080";

    // Create the server and bind it to the address
    HttpServer::new(move || {
        App::new()
            .data(mongodb_client.clone())
            .service(index)
            .service(web::scope("/api").configure(api::init_routes))
    })
    .bind(addr)?
    .run()
    .await
}
