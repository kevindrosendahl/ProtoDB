use super::QueryPlannerError;

use std::result;

pub type Result<T> = result::Result<T, QueryPlannerError>;
