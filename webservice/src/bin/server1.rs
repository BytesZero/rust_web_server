use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};

// 配置路由
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/health").route(web::get().to(health)));
}

// 处理请求
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world actix-web!")
}

// 健康检查
async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

// 启动服务
#[actix_rt::main]
async fn main() -> Result<(), Error> {
    let app = move || App::new().configure(general_routes);
    println!("Server running at http://localhost:8080/");
    HttpServer::new(app).bind("0.0.0.0:8080")?.run().await?;
    Ok(())
}
