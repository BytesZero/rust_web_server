use actix_web::{
    error::{self, Error},
    http::StatusCode,
    web::JsonConfig,
    HttpResponse,
};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub code: i16,
    pub msg: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppError {{ code: {}, msg: {} }}", self.code, self.msg)
    }
}

impl AppError {
    pub fn json_error(config: JsonConfig) -> JsonConfig {
        config.error_handler(|_, _req| {
            let app_err = AppError {
                code: 400,
                msg: String::from("请求参数错误"),
            };
            Error::from(app_err)
        })
    }
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppError {
            code: self.code,
            msg: self.msg.clone(),
        })
    }
}
