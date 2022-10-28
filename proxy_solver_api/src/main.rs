#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;

#[get("/health/ready")]
fn health_ready() -> Status {
    Status::NoContent
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct AttendanceSnapshot {
    members: Vec<String>,
    members_present: Vec<String>,
}

#[post("/solution", data = "<snapshot>")]
fn solution(snapshot: Json<AttendanceSnapshot>) -> Value {
    json!({
        "members": snapshot.members.len(),
        "members_present": snapshot.members_present.len(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health_ready, solution])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn test_health_ready() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::health_ready)).dispatch();
        assert_eq!(response.status(), Status::NoContent);
    }

    #[test]
    fn test_solution_counts() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!(super::solution)).header(ContentType::JSON).body(
            r#"{"members": ["L. L. Nunn", "Herbert Reich"], "members_present": ["Herbert Reich"]}"#,
        ).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            r#"{"members":2,"members_present":1}"#
        )
    }
}
