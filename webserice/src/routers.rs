use super::handlers::*;
use actix_web::web;

// 配置路由
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/health").route(web::get().to(health)));
}

// 课程路由
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("course")
            // .route("/add", web::post().to(course_add))
            // .route("/del/{id}", web::delete().to(course_del))
            // .route("/update", web::put().to(course_update))
            .route("/get/{id}", web::get().to(course_get)), // .route("/list", web::get().to(course_list)),
    );
}
