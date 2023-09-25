use sea_orm::DatabaseConnection;

// use super::models::Course;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub course: Mutex<Vec<Course>>,
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        AppState {
            health_check_response: "ok".to_string(),
            visit_count: Mutex::new(0),
            // course: Mutex::new(Vec::new()),
            db: db,
        }
    }
}
