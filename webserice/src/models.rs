use super::apperror::AppError;
use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::{Deserialize, Serialize};

// 统一返回参数
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response<'a, T> {
    pub code: i16,
    pub msg: &'a str,
    pub data: Option<T>,
}
// 统一返回成功
pub fn resp<T: Serialize>(
    respb: &mut HttpResponseBuilder,
    code: i16,
    msg: &str,
    data: Option<T>,
) -> Result<HttpResponse, AppError> {
    Ok(respb.json(Response { code, msg, data }))
}
// 返回成功，默认code为0
pub fn resp_ok<T: Serialize>(msg: &str, data: Option<T>) -> Result<HttpResponse, AppError> {
    resp(&mut HttpResponse::Ok(), 0, msg, data)
}
// 返回成功，无data,默认code为0
pub fn resp_ok_none(msg: &str) -> Result<HttpResponse, AppError> {
    resp_ok(msg, None::<()>)
}

// 返回成功但处理有错误，无data
// pub fn resp_ok_err(code: i16, msg: &str) -> Result<HttpResponse, AppError> {
//     resp(&mut HttpResponse::Ok(), code, msg, None::<()>)
// }

// 统一返回失败
pub fn resp_err(
    respb: &mut HttpResponseBuilder,
    code: i16,
    msg: &str,
) -> Result<HttpResponse, AppError> {
    resp(respb, code, msg, None::<()>)
}

pub fn resp_err404(msg: &str) -> Result<HttpResponse, AppError> {
    resp_err(&mut HttpResponse::NotFound(), 404, msg)
}
