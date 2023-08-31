#![warn(missing_docs)]
//! Rocket server and API endpoints.
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
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes};

use rocket_slogger::Slogger;

#[openapi(ignore = "log")]
#[get("/health/ready")]
/// Wakes up the API, for example if sleeping at Heroku.
fn get_health_ready(log: Slogger) -> Status {
    info!(log, "Ready");
    Status::NoContent
}

#[openapi(ignore = "log")]
#[post("/solution", data = "<problem>")]
/// Given a [`problem::ProxyProblem`], computes and returns the
/// [`solution::ProxySolution`].  `ProxySolution` is deterministic and constant
/// for a given `ProxyProblem`.
fn post_solution(log: Slogger, problem: Json<ProxyProblem>) -> Json<ProxySolution> {
    let mut solution = ProxySolution::from_problem(&problem);

    info!(log, "Beginning solution"; "metrics" => solution.get_metrics());
    solution.solve(&log);

    info!(log, "Solved"; "metrics" => solution.get_metrics());
    Json(solution)
}

#[launch]
/// Sets up Rocket routing, plus extras for logging and OpenAPI generation.
fn rocket() -> _ {
    let fairing = Slogger::new_bunyan_logger(env!("CARGO_PKG_NAME"));

    let mut config = Config::from(Config::figment());
    config.log_level = LogLevel::Off;

    rocket::build()
        .attach(fairing)
        .mount("/", openapi_get_routes![get_health_ready, post_solution])
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
