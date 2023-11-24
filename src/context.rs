use chrono::{DateTime, Utc};
use sqlx::{mysql::MySql, pool::Pool};

#[derive(Debug)]
pub struct AppState {
    pub start_time: DateTime<Utc>,
    pub execution_id: String,
    pub pool: Pool<MySql>,
}

impl AppState {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self {
            start_time: Utc::now(),
            execution_id: nanoid::nanoid!(),
            pool,
        }
    }
}
