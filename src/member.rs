//! [`Member`]s and [`MemberId`]s are the building blocks of
//! [`crate::problem::ProxyProblem`]s and [`crate::solution::ProxySolution`]s.
use indexmap::map::IndexMap;

use matchmaker::{Category, Student};

use rocket::serde::Deserialize;
use rocket_okapi::JsonSchema;

/// An opaque [`String`] that uniquely keys a [`Member`] and identifies their
/// [`MemberInfo`]`.preferences`.
pub type MemberId = String;

/// Alias for `matchmaker::Category`.
pub type Proxy = Category;

/// A `matchmaker::Student`.
pub trait Member {
    /// A [`Member`]'s preferences are exclusive: they MAY be represented only
    /// by a member whose [`MemberId`] they list in their
    /// [`MemberInfo`]`.preferences`; they MUST NOT be represented by any other
    /// member.
    ///
    /// Furthermore, since a member CANNOT be represented by an absent member,
    /// we only store preferences corresponding to present members.
    fn from_info(info: &MemberInfo, present: &IndexMap<MemberId, Category>) -> Student;
}

impl Member for Student {
    fn from_info(info: &MemberInfo, present: &IndexMap<MemberId, Category>) -> Student {
        Student {
            name: info.id.clone(),
            preferences: info
                .preferences
                .iter()
                .filter_map(|k| present.get(k))
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

#[derive(Deserialize, JsonSchema)]
/// Stores members' preference contraints.
pub struct MemberInfo {
    /// In graph terms, the member's node.
    pub id: MemberId,
    /// In graph terms, a ranked list of the member's edges to other members who
    /// MAY represent them.
    pub preferences: Vec<MemberId>,
}
