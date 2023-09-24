#[path = "app_error.rs"]
mod apperror;
#[path = "handlers.rs"]
mod handlers;
#[path = "models.rs"]
mod models;
#[path = "routers.rs"]
mod routers;
#[path = "state.rs"]
mod state;

use actix_web::{
    web::{self, JsonConfig},
    App, Error, HttpServer,
};
use routers::*;
use state::*;

use crate::apperror::AppError;

// 启动服务
#[actix_rt::main]
async fn main() -> Result<(), Error> {
    let share_data = web::Data::new(AppState::new());
    let app = move || {
        App::new()
            .app_data(share_data.clone())
            .app_data(AppError::json_error(JsonConfig::default()))
            .configure(general_routes)
            .configure(course_routes)
    };
    println!("Server running at http://localhost:8080/");
    HttpServer::new(app).bind("0.0.0.0:8080")?.run().await?;
    Ok(())
}
