//! A solution to a [`ProxyProblem`].
use crate::member::{Member, MemberId, MemberInfo, Proxy};
use crate::metrics::ProxyMetrics;
use crate::problem::ProxyProblem;

use indexmap::map::IndexMap;
use indexmap::set::IndexSet;
use matchmaker::da_stb::match_students;
use matchmaker::Student;
use rand::{rngs::StdRng, SeedableRng};

use rocket_okapi::JsonSchema;
use rocket_slogger::Slogger;
use serde::Serialize;

#[derive(new, JsonSchema, Serialize)]
/// A pending or computed solution to a [`ProxyProblem`].
pub struct ProxySolution {
    #[serde(skip)]
    /// From [`ProxyProblem`]: How many absent members each present member MAY
    /// represent.
    pub capacity: usize,

    #[new(default)]
    #[serde(skip)]
    /// A present member is a [`Proxy`] that can represent up to `capacity`
    /// member/s.
    pub members_present: IndexMap<MemberId, Proxy>,

    #[new(default)]
    #[serde(skip)]
    /// An absent member is a `matchmaker::Student` for sizing purposes.
    pub members_absent: IndexMap<MemberId, Student>,

    #[new(default)]
    /// `A` → `P` for absent member A represented by present member P.
    pub members_represented: IndexMap<MemberId, MemberId>,

    #[new(default)]
    /// Absent members who are not represented in this solution.
    pub members_unrepresented: IndexSet<MemberId>,
}

impl ProxySolution {
    /// Loads the constraints of a [`ProxyProblem`] to be solved.
    pub fn from_problem(problem: &ProxyProblem) -> Self {
        let mut solution = Self::new(problem.capacity);
        solution.load_attendance(&problem.members_present);
        solution.load_preferences(&problem.members);

        solution
    }

    /// Returns [`ProxyMetrics`] of the solution *in its current state* (whether
    /// solved or unsolved).
    pub fn metrics(&self) -> ProxyMetrics {
        ProxyMetrics {
            capacity: self.capacity,
            total: self.members_present.len() + self.members_absent.len(),
            present: self.members_present.len(),
            absent: self.members_absent.len(),
            represented: self.members_represented.len(),
            unrepresented: self.members_unrepresented.len(),
        }
    }

    /// Computes the solution to the loaded [`ProxyProblem`].
    ///
    /// Under the hood, this is `matchmaker::da_stb::match_students()` with a
    /// zero-seeded PRNG for deterministic solutions.
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

        // Flatten each [`Proxy`]) of `P` → `{A1, A2, ...}` into `A1` → `P`,
        // `A2` → `P`, ... .
        for present in self.members_present.values() {
            if let Some(assigned) = result.placed.get(&present.name) {
                for absent in assigned {
                    self.members_unrepresented.remove(&absent.name);
                    self.members_represented
                        .insert(absent.name.clone(), present.name.clone());
                    debug!(log, "Proxy assigned"; "proxy_for" => absent.name.clone(), "proxied_by" => present.name.clone());
                }
            }
        }

        self.members_represented.sort_keys();
        self.members_unrepresented.sort();
    }

    /// Sets the *presence* constraints on the [`ProxySolution`].
    ///
    /// Converts present [`MemberId`]s into available [`Proxy`]s.
    fn load_attendance(&mut self, members_present: &Vec<MemberId>) {
        for id in members_present {
            let present = Proxy::new(id, self.capacity);
            self.members_present.insert(id.clone(), present);
        }
    }

    /// Sets the *preference* constraints on the [`ProxySolution`].
    ///
    /// Converts all [`MemberInfo`]s into [`Member`]s.
    fn load_preferences(&mut self, members: &Vec<MemberInfo>) {
        for info in members {
            if !self.members_present.contains_key(&info.id) {
                let absent = Student::new_from_info(info, &self.members_present);
                self.members_absent.insert(info.id.clone(), absent);
                self.members_unrepresented.insert(info.id.clone());
            }
        }
    }
}
