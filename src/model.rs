use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize, Serialize)]
pub struct Food {
    pub id: Option<Thing>,
    pub name: String,
    pub brand: String,
    pub cost: usize,
    pub weight: usize,
    pub volume: usize,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}
