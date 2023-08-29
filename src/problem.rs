use crate::member::{MemberId, MemberInfo};

use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct ProxyProblem {
    pub capacity: usize,
    pub members: Vec<MemberInfo>,
    pub members_present: Vec<MemberId>,
}
