#[macro_use]
extern crate matchmaker;
#[macro_use]
extern crate rocket;
use matchmaker::{Category, Student};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;
use std::collections::HashMap;

#[get("/health/ready")]
fn health_ready() -> Status {
    Status::NoContent
}

#[derive(Deserialize)]
struct MemberInfo {
    id: String,
    preferences: Vec<String>,
}

trait Member {
    fn from_info(info: &MemberInfo, present: &HashMap<String, Category>) -> Student;
}

impl Member for Student {
    fn from_info(info: &MemberInfo, present: &HashMap<String, Category>) -> Student {
        Student {
            name: info.id.clone(),
            preferences: info
                .preferences
                .iter()
                .filter(|&k| present.contains_key(k))
                .map(|k| present.get(k).unwrap())
                .cloned()
                .collect(),
            exclude: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
struct AttendanceSnapshot {
    members: Vec<MemberInfo>,
    members_present: Vec<String>,
}

#[post("/solution", data = "<snapshot>")]
fn solution(snapshot: Json<AttendanceSnapshot>) -> Value {
    let mut members_present: HashMap<String, Category> = HashMap::new();
    for id in &snapshot.members_present {
        let member = Category::new(&id, 2); // FIXME: constant
        members_present.insert(id.clone(), member);
    }

    let mut members: HashMap<String, Student> = HashMap::new();
    for info in &snapshot.members {
        let member = <Student as Member>::from_info(info, &members_present);
        members.insert(info.id.clone(), member);
    }

    println! {"Members ({}): {:?}", members.len(), members.keys()}
    println! {"Present ({}): {:?}", members_present.len(), members_present.keys()}

    json!({
        "members": members.len(),
        "members_present": members_present.len(),
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
        let response = client
            .post(uri!(super::solution))
            .header(ContentType::JSON)
            .body(
                r#"{
                "members": [
                    {"id": "nunn", "preferences": ["reich", "whitney"]},
                    {"id": "reich", "preferences": []},
                    {"id": "whitney", "preferences": []}
                ],
                "members_present": [
                    "reich",
                    "whitney"
                ]
            }"#,
            )
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            r#"{"members":3,"members_present":2}"#
        )
    }
}
