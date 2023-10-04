//! Metrics for how much representation the [`crate::problem::ProxyProblem`] or
//! [`crate::solution::ProxySolution`] has achieved.
use serde::Serialize;

#[derive(Clone, SerdeValue, Serialize)]
/// Metrics for how much representation the [`crate::problem::ProxyProblem`] or
/// [`crate::solution::ProxySolution`] has achieved.  Producers MUST calculate
/// these values based on their own internal state.
pub struct ProxyMetrics {
    /// How many absent members each present member MAY represent.
    pub capacity: usize,
    /// How many members total.
    pub total: usize,
    /// How many members present.
    pub present: usize,
    /// How many members absent.
    pub absent: usize,
    /// How many members are represented in this solution.
    pub represented: usize,
    /// How many members cannot be represented in this solution.
    pub unrepresented: usize,
}
