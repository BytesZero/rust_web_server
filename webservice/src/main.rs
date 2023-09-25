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

use std::env;

use actix_web::{
    web::{self, JsonConfig},
    App, Error, HttpServer,
};
use dotenvy;
use routers::*;
use sea_orm::Database;
use state::*;

use crate::apperror::AppError;

// 启动服务
#[actix_rt::main]
async fn main() -> Result<(), Error> {
    // 读取配置文件
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // 连接数据库
    let conn = Database::connect(&db_url).await.unwrap();
    // 启动 Web 服务
    let share_data = web::Data::new(AppState::new(conn));
    let app = move || {
        App::new()
            .app_data(share_data.clone())
            .app_data(AppError::json_error(JsonConfig::default()))
            .configure(default_routes)
            .configure(course_routes)
    };
    println!("Server running at http://localhost:8080/");
    HttpServer::new(app).bind("0.0.0.0:8080")?.run().await?;
    Ok(())
}
