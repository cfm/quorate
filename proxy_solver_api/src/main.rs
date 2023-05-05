#[macro_use]
extern crate matchmaker;
#[macro_use]
extern crate rocket;
use indexmap::map::IndexMap;
use indexmap::set::IndexSet;
use matchmaker::da_stb::match_students;
use matchmaker::{Category, Student};
use rand::{rngs::StdRng, SeedableRng};
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;

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
    fn from_info(info: &MemberInfo, present: &IndexMap<String, Category>) -> Student;
}

impl Member for Student {
    fn from_info(info: &MemberInfo, present: &IndexMap<String, Category>) -> Student {
        Student {
            name: info.id.clone(),
            preferences: info
                .preferences
                .iter()
                .filter(|&k| present.contains_key(k))
                .map(|k| present.get(k).unwrap())
                .cloned()
                .collect(),
            exclude: present
                .values()
                .filter(|&v| !info.preferences.contains(&v.name))
                .map(|v| present.get(&v.name).unwrap())
                .cloned()
                .collect(),
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
    let mut presents: IndexMap<String, Category> = IndexMap::new();
    for id in &snapshot.members_present {
        let present = Category::new(&id, 2); // FIXME: constant
        presents.insert(id.clone(), present);
    }

    let mut absents: IndexMap<String, Student> = IndexMap::new();
    let mut unrepresented: IndexSet<String> = IndexSet::new();
    for info in &snapshot.members {
        if !presents.contains_key(&info.id) {
            let absent = <Student as Member>::from_info(info, &presents);
            absents.insert(info.id.clone(), absent);
            unrepresented.insert(info.id.clone());
        }
    }

    println! {"absent={} members={:?}", absents.len(), absents.keys()}
    println! {"present={} members={:?}", presents.len(), presents.keys()}

    let mut rng = StdRng::seed_from_u64(0);
    let result = match_students(
        absents.clone().into_values().collect(),
        &Vec::from(presents.clone().into_values().collect::<Vec<_>>()),
        &mut rng,
    );

    let mut proxies: IndexMap<String, String> = IndexMap::new();
    for present in presents.values() {
        for absent in result.placed.get(&present.name).unwrap_or(&Vec::new()) {
            unrepresented.remove(&absent.name);
            proxies.insert(absent.name.clone(), present.name.clone());
            println!("{} → {}", &present.name, &absent.name);
        }
    }

    proxies.sort_keys();
    unrepresented.sort();

    json!({
        "represented": proxies,
        "unrepresented": unrepresented,
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
    use std::path::PathBuf;
    use std::{env, fs};

    #[test]
    fn test_health_ready() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::health_ready)).dispatch();
        assert_eq!(response.status(), Status::NoContent);
    }

    #[test]
    fn test_first_choice_available() {
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
            r#"{"represented":{"nunn":"reich"},"unrepresented":[]}"#
        )
    }

    #[test]
    fn test_second_choice_available() {
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
                    "whitney"
                ]
            }"#,
            )
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            r#"{"represented":{"nunn":"whitney"},"unrepresented":["reich"]}"#
        )
    }

    #[test]
    fn test_big() {
        let mut request_from = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        request_from.push("resources/tests/test_big.json");
        let request = fs::read(request_from).unwrap();

        let mut expected_from = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        expected_from.push("resources/tests/test_big.result.json");
        let expected: String = fs::read_to_string(expected_from)
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(super::solution))
            .header(ContentType::JSON)
            .body(&request)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), expected,)
    }
}
