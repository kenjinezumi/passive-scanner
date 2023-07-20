// main.rs

mod handlers;
use handlers::{index, scan, ClientManager};

use rocket::State;
use reqwest::blocking::Client;
use rocket::fs::FileServer;
use rocket::routes;
use rocket::response::content::Html; // Add this import

use tokio::runtime::Runtime;



fn main() {
    let client = Client::new();
    let client_manager = ClientManager { client };

    rocket::build()
        .mount("/", routes![index, scan])
        .manage(client_manager)
        .mount("/static", FileServer::from("static")) // Serve static files from the "static" directory
        .launch();
}