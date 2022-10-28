#[macro_use]
extern crate rocket;
use rocket::http::Status;

#[get("/health/ready")]
fn health_ready() -> Status {
    Status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_ready])
}
