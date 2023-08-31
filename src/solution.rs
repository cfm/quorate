//! A solution to a [`crate::problem::ProxyProblem`].
use crate::member::{Member, MemberId, MemberInfo, Proxy};
use crate::problem::ProxyProblem;

use indexmap::map::IndexMap;
use indexmap::set::IndexSet;
use matchmaker::da_stb::match_students;
use matchmaker::Student;
use rand::{rngs::StdRng, SeedableRng};

use rocket::serde::Serialize;
use rocket_okapi::JsonSchema;
use rocket_slogger::Slogger;

#[derive(Clone, SerdeValue, Serialize)]
/// Metrics for how much representation the [`crate::solution::ProxySolution`]
/// has achieved.
pub struct ProxyMetrics {
    /// From [`crate::problem::ProxyProblem`]: How many absent members each
    /// present member MAY represent.
    pub capacity: usize,
    /// From [`crate::problem::ProxyProblem`]: How many members total.
    pub total: usize,
    /// From [`crate::problem::ProxyProblem`]: How many members present.
    pub present: usize,
    /// From [`crate::problem::ProxyProblem`]: How many members absent.
    pub absent: usize,
    /// From [`ProxySolution`]: How many members are represented in this
    /// solution.
    pub represented: usize,
    /// From [`ProxySolution`]: How many members cannot be represented in this
    /// solution.
    pub unrepresented: usize,
}

#[derive(new, JsonSchema, Serialize)]
/// A pending or computed solution to a [`crate::problem::ProxyProblem`].
pub struct ProxySolution {
    #[serde(skip)]
    /// From [`crate::problem::ProxyProblem`]: How many absent members each
    /// present member MAY represent.
    pub capacity: usize,

    #[new(default)]
    #[serde(skip)]
    /// A present member is a [`crate::member::Proxy`] that can represent up to
    /// `capacity` member/s.
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
    /// Loads the constraints of a [`crate::problem::ProxyProblem`] to be solved.
    pub fn from_problem(problem: &ProxyProblem) -> Self {
        let mut solution = Self::new(problem.capacity);
        solution.load_attendance(&problem.members_present);
        solution.load_preferences(&problem.members);

        solution
    }

    /// Gets [`ProxyMetrics`] of the solution *in its current state* (whether
    /// solved or unsolved).
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

    /// Computes the solution to the loaded [`crate::problem::ProxyProblem`].
    ///
    /// Under the hood, this is `matchmaker::match_students()` with a
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

    /// Sets the *presence* constraints on the [`ProxySolution`].
    ///
    /// Converts present [`crate::member::MemberId`]s into available
    /// [`crate::member::Proxy`]s.
    fn load_attendance(&mut self, members_present: &Vec<MemberId>) {
        for id in members_present {
            let present = Proxy::new(id, self.capacity);
            self.members_present.insert(id.clone(), present);
        }
    }

    /// Sets the *preference* constraints on the [`ProxySolution`].
    ///
    /// Converts all [`crate::member::MemberInfo`]s into
    /// [`crate::member::Member`]s.
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
