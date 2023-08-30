use crate::member::{Member, MemberId, MemberInfo};
use crate::problem::ProxyProblem;

use indexmap::map::IndexMap;
use indexmap::set::IndexSet;
use matchmaker::da_stb::match_students;
use matchmaker::{Category, Student};
use rand::{rngs::StdRng, SeedableRng};

use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;
use rocket_slogger::Slogger;

#[derive(Clone, SerdeValue, Serialize)]
pub struct ProxyMetrics {
    pub capacity: usize,
    pub total: usize,
    pub present: usize,
    pub absent: usize,
    pub represented: usize,
    pub unrepresented: usize,
}

#[derive(new, JsonSchema, Serialize)]
pub struct ProxySolution {
    #[serde(skip)]
    pub capacity: usize,

    #[new(default)]
    #[serde(skip)]
    pub members_present: IndexMap<MemberId, Category>,

    #[new(default)]
    #[serde(skip)]
    pub members_absent: IndexMap<MemberId, Student>,

    #[new(default)]
    pub members_represented: IndexMap<MemberId, MemberId>,

    #[new(default)]
    pub members_unrepresented: IndexSet<MemberId>,
}

impl ProxySolution {
    pub fn from_problem(problem: &ProxyProblem) -> Self {
        let mut solution = Self::new(problem.capacity);
        solution.load_attendance(&problem.members_present);
        solution.load_preferences(&problem.members);

        solution
    }

    pub fn get_metrics(&mut self) -> ProxyMetrics {
        ProxyMetrics {
            capacity: self.capacity,
            total: self.members_present.len() + self.members_absent.len(),
            present: self.members_present.len(),
            absent: self.members_absent.len(),
            represented: self.members_represented.len(),
            unrepresented: self.members_unrepresented.len(),
        }
    }

    pub fn solve(&mut self, log: &Slogger) {
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
}
