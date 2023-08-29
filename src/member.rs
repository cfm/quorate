use indexmap::map::IndexMap;

use matchmaker::{Category, Student};

use rocket::serde::Deserialize;

pub type MemberId = String;

pub trait Member {
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
pub struct MemberInfo {
    pub id: MemberId,
    pub preferences: Vec<MemberId>,
}
