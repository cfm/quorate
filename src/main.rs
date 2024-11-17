#![deny(clippy::all)]
#![deny(rustdoc::all)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

use crate::problem::{solve, ProxyProblem};
use crate::solution::ProxySolution;

pub mod member;
pub mod metrics;
pub mod problem;
pub mod solution;

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_slogger;
#[macro_use]
extern crate slog_derive;

use rocket::config::Config;
use rocket::http::Status;
use rocket::log::LogLevel;
use rocket::serde::json;
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
    info!(log, "Defined problem"; "metrics" => problem.metrics());
    let solution = solve(&problem, &log);
    info!(log, "Found solution"; "metrics" => solution.metrics());
    Json(solution)
}

#[launch]
#[cfg(feature = "rocket")]
/// Sets up Rocket routing, plus extras for logging and OpenAPI generation.
fn rocket() -> _ {
    let fairing = Slogger::new_bunyan_logger(env!("CARGO_PKG_NAME"));

    let mut config = Config::from(Config::figment());
    config.log_level = LogLevel::Off;

    rocket::build()
        .attach(fairing)
        .mount("/", openapi_get_routes![get_health_ready, post_solution])
}

use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::HeaderMap;
use http::Method;
use lambda_runtime::{handler_fn, Context, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
#[cfg(not(feature = "rocket"))]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[cfg(not(feature = "rocket"))]
pub(crate) async fn handler(
    event: ApiGatewayProxyRequest,
    _ctx: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let logger = Slogger::new_bunyan_logger(env!("CARGO_PKG_NAME"));

    let res = match (event.http_method, event.path.unwrap().as_str()) {
        (Method::GET, path) => ApiGatewayProxyResponse {
            status_code: get_health_ready(logger).code as i64,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Empty),
            is_base64_encoded: Some(false),
        },
        (Method::POST, "/solution") => {
            let problem: ProxyProblem = json::from_str(&event.body.unwrap()).unwrap();

            let solution = post_solution(logger, Json(problem));
            // NB. T is .0 in Json<T> (https://api.rocket.rs/v0.5-rc/rocket/serde/json/struct.Json.html#structfield.0).
            let body = json::to_pretty_string(&solution.0).unwrap();

            ApiGatewayProxyResponse {
                status_code: 200,
                headers: HeaderMap::new(),
                multi_value_headers: HeaderMap::new(),
                body: Some(Body::Text(body)),
                is_base64_encoded: Some(false),
            }
        }
        _ => ApiGatewayProxyResponse {
            status_code: 400,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Empty),
            is_base64_encoded: Some(false),
        },
    };

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;
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
