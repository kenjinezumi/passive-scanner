use rocket::form::Form;
use rocket::State;
use rocket::response::content::Html; // Add this import

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
    pub client: Client,
}

#[get("/")]
pub fn index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
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




