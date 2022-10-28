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

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_health_ready() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::health_ready)).dispatch();
        assert_eq!(response.status(), Status::NoContent);
    }
}
