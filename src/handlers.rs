use rocket::form::Form;
use rocket::State;
use reqwest::blocking::Client;
use serde::{Serialize};
use rocket::serde::json::Json;
use rocket::get;
use rocket::post;
use rocket::FromForm;
use rocket::routes;

#[derive(FromForm)]
pub struct ScanForm {
    url: String,
}

#[derive(Serialize)]
pub struct ScanResult {
    url: String,
    result: String,
}

pub struct ClientManager {
    client: Client,
}

#[get("/")]
pub fn index() -> &'static str {
    "Welcome to the Passive Scanner Web Application!"
}

#[post("/scan", data = "<scan_form>")]
pub fn scan(scan_form: Form<ScanForm>, client_manager: &State<ClientManager>) -> Json<ScanResult> {
    let url = &scan_form.url;
    let response = client_manager.client
        .get(url)
        .send()
        .expect("Failed to send request");

    let mut result = String::new();
    if response.status().is_success() {
        result += &format!("<h2>Scanning URL: {}</h2>", url);
        result += "Add your analysis logic here...";
    } else {
        result = format!(
            "<h2>Request to '{}' failed with status code: {}</h2>",
            url,
            response.status()
        );
    }

    Json(ScanResult {
        url: url.to_string(),
        result,
    })
}

#[rocket::main]
async fn main()  {
    let client = Client::new();
    let client_manager = ClientManager { client };

    rocket::build()
        .mount("/", routes![index, scan])
        .manage(client_manager)
        .launch();
}

