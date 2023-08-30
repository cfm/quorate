use crate::member::{MemberId, MemberInfo};

use rocket::serde::Deserialize;
use rocket_okapi::JsonSchema;

#[derive(Deserialize, JsonSchema)]
pub struct ProxyProblem {
    pub capacity: usize,
    pub members: Vec<MemberInfo>,
    pub members_present: Vec<MemberId>,
}
