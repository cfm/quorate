use crate::problem::ProxyProblem;
use crate::solution::ProxySolution;

pub mod member;
pub mod problem;
pub mod solution;

#[macro_use]
extern crate derive_new;
extern crate matchmaker;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_slogger;
#[macro_use]
extern crate slog_derive;

use rocket::config::Config;
use rocket::http::Status;
use rocket::log::LogLevel;
use rocket::serde::json::{json, Json, Value};

use rocket_slogger::Slogger;

#[get("/health/ready")]
fn get_health_ready(log: Slogger) -> Status {
    info!(log, "Ready");
    Status::NoContent
}

#[post("/solution", data = "<problem>")]
fn post_solution(log: Slogger, problem: Json<ProxyProblem>) -> Value {
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
        .mount("/", routes![get_health_ready, post_solution])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn test_health_ready() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::get_health_ready)).dispatch();
        assert_eq!(response.status(), Status::NoContent);
    }

    #[test]
    fn test_first_choice_available() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(super::post_solution()))
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
            .post(uri!(super::post_solution()))
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
        let request = include_str!("../resources/tests/test_big.json");
        let expected: String = include_str!("../resources/tests/test_big.result.json")
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post(uri!(super::post_solution()))
            .header(ContentType::JSON)
            .body(&request)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), expected,)
    }
}
