//! A proxy-representation problem to solve.
use crate::member::{MemberId, MemberInfo};

use rocket::serde::Deserialize;
use rocket_okapi::JsonSchema;

#[derive(Deserialize, JsonSchema)]
/// A proxy-representation problem to solve.
pub struct ProxyProblem {
    /// How many absent members each present member MAY represent.
    pub capacity: usize,
    /// All members, present and absent.
    pub members: Vec<MemberInfo>,
    /// Present members only, who CAN represent absent members.
    pub members_present: Vec<MemberId>,
}
