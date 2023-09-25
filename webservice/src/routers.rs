use super::handlers::*;
use actix_web::web;

// 默认路由
pub fn default_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index).service(health);
}

// 课程路由
pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("course")
            .service(course_add)
            .service(course_del)
            .service(course_update)
            .service(course_get)
            .service(course_list),
    );
}
