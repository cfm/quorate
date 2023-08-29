#[macro_use]
extern crate derive_new;
extern crate matchmaker;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_slogger;
#[macro_use]
extern crate slog_derive;
use indexmap::map::IndexMap;
use indexmap::set::IndexSet;
use matchmaker::da_stb::match_students;
use matchmaker::{Category, Student};
use rand::{rngs::StdRng, SeedableRng};
use rocket::config::Config;
use rocket::http::Status;
use rocket::log::LogLevel;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket_slogger::Slogger;

#[get("/health/ready")]
fn health_ready(log: Slogger) -> Status {
    info!(log, "Ready");
    Status::NoContent
}

type MemberId = String;

#[derive(Deserialize)]
struct MemberInfo {
    id: MemberId,
    preferences: Vec<MemberId>,
}

trait Member {
    fn from_info(info: &MemberInfo, present: &IndexMap<MemberId, Category>) -> Student;
}

impl Member for Student {
    fn from_info(info: &MemberInfo, present: &IndexMap<MemberId, Category>) -> Student {
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
struct ProxyProblem {
    capacity: usize,
    members: Vec<MemberInfo>,
    members_present: Vec<MemberId>,
}

#[derive(new, Serialize)]
struct ProxySolution {
    #[serde(skip)]
    capacity: usize,

    #[new(default)]
    #[serde(skip)]
    members_present: IndexMap<MemberId, Category>,

    #[new(default)]
    #[serde(skip)]
    members_absent: IndexMap<MemberId, Student>,

    #[new(default)]
    members_represented: IndexMap<MemberId, MemberId>,

    #[new(default)]
    members_unrepresented: IndexSet<MemberId>,
}

#[derive(Clone, SerdeValue, Serialize)]
struct ProxyMetrics {
    capacity: usize,
    total: usize,
    present: usize,
    absent: usize,
    represented: usize,
    unrepresented: usize,
}

impl ProxySolution {
    fn from_problem(problem: &ProxyProblem) -> Self {
        let mut solution = Self::new(problem.capacity);
        solution.load_attendance(&problem.members_present);
        solution.load_preferences(&problem.members);

        solution
    }

    fn get_metrics(&mut self) -> ProxyMetrics {
        ProxyMetrics {
            capacity: self.capacity,
            total: self.members_present.len() + self.members_absent.len(),
            present: self.members_present.len(),
            absent: self.members_absent.len(),
            represented: self.members_represented.len(),
            unrepresented: self.members_unrepresented.len(),
        }
    }

    fn load_attendance(&mut self, members_present: &Vec<MemberId>) {
        for id in members_present {
            let present = Category::new(id, self.capacity);
            self.members_present.insert(id.clone(), present);
        }
    }

    fn load_preferences(&mut self, members: &Vec<MemberInfo>) {
        for info in members {
            if !self.members_present.contains_key(&info.id) {
                let absent = <Student as Member>::from_info(info, &self.members_present);
                self.members_absent.insert(info.id.clone(), absent);
                self.members_unrepresented.insert(info.id.clone());
            }
        }
    }

    fn solve(&mut self, log: &Slogger) {
        let mut rng = StdRng::seed_from_u64(0);
        let result = match_students(
            self.members_absent.clone().into_values().collect(),
            &self
                .members_present
                .clone()
                .into_values()
                .collect::<Vec<_>>(),
            &mut rng,
        );

        for present in self.members_present.values() {
            for absent in result.placed.get(&present.name).unwrap_or(&Vec::new()) {
                self.members_unrepresented.remove(&absent.name);
                self.members_represented
                    .insert(absent.name.clone(), present.name.clone());
                debug!(log, "Proxy assigned"; "proxy_for" => absent.name.clone(), "proxied_by" => present.name.clone());
            }
        }

        self.members_represented.sort_keys();
        self.members_unrepresented.sort();
    }
}

#[post("/solution", data = "<problem>")]
fn solution(log: Slogger, problem: Json<ProxyProblem>) -> Value {
    let mut solution = ProxySolution::from_problem(&problem);

    info!(log, "Beginning solution"; "metrics" => solution.get_metrics());
    solution.solve(&log);

    info!(log, "Solved"; "metrics" => solution.get_metrics());
    json!(solution)
}

#[launch]
fn rocket() -> _ {
    let fairing = Slogger::new_bunyan_logger(env!("CARGO_PKG_NAME"));

    let mut config = Config::from(Config::figment());
    config.log_level = LogLevel::Off;

    rocket::build()
        .attach(fairing)
        .mount("/", routes![health_ready, solution])
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
            .post(uri!(super::solution()))
            .header(ContentType::JSON)
            .body(
                r#"{
                "capacity": 2,
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
            r#"{"members_represented":{"nunn":"reich"},"members_unrepresented":[]}"#
        )
    }

    #[test]
    fn test_second_choice_available() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(super::solution()))
            .header(ContentType::JSON)
            .body(
                r#"{
                "capacity": 2,
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
            r#"{"members_represented":{"nunn":"whitney"},"members_unrepresented":["reich"]}"#
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
            .post(uri!(super::solution()))
            .header(ContentType::JSON)
            .body(&request)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), expected,)
    }
}
