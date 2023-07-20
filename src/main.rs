use rocket::routes;

mod handlers;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ handlers::scan])
}
