//! A proxy-representation problem to solve.
use crate::member::{MemberId, MemberInfo};
use crate::metrics::ProxyMetrics;
use crate::solution::ProxySolution;

use rocket_okapi::JsonSchema;
use rocket_slogger::Slogger;
use serde::Deserialize;

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

impl ProxyProblem {
    /// Returns [`ProxyMetrics`] of the problem before any solution has been
    /// attempted.
    pub fn metrics(&self) -> ProxyMetrics {
        ProxyMetrics {
            capacity: self.capacity,
            total: self.members.len(),
            present: self.members_present.len(),
            absent: self.members.len() - self.members_present.len(),
            represented: 0,
            unrepresented: 0,
        }
    }
}

/// Solve the given [`ProxyProblem`].  This is a wrapper to hide the
/// statefulness of the [`ProxySolution`] pending further refactoring[1].
///
/// [1]: https://github.com/cfm/proxy-solver-api/pull/9#discussion_r1312893812
pub fn solve(problem: &ProxyProblem, log: &Slogger) -> ProxySolution {
    let mut solution = ProxySolution::from_problem(problem);
    solution.solve(log);

    solution
}
