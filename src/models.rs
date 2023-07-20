use rocket::form::FromForm;

#[derive(FromForm)]
pub struct ScanForm {
    pub url: String,
}
