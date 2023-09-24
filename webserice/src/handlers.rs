use super::apperror::AppError;
use super::models::*;
use super::state::*;
use ::entity::course::ActiveModel;
use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
// use entity::{course, course::Entity as Course};
use ::entity::{course, course::Entity as Course};
use sea_orm::*;

// 执行index函数
pub async fn index() -> Result<HttpResponse, AppError> {
    resp_ok_none("Hello world actix-web!")
}

// 执行health函数
pub async fn health(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;
    println!("visit_count: {}", visit_count);
    let resp = format!("{}: {}", health_check_response, visit_count);
    resp_ok_none(resp.as_str())
}

// 执行course_add函数
pub async fn course_add(
    app_state: web::Data<AppState>,
    course: web::Json<course::Model>,
) -> Result<HttpResponse, AppError> {
    let mut course: ActiveModel = course.into_inner().into();
    course.create_time = Set(Some(NaiveDateTime::default()));
    let result = Course::insert(course).exec(&app_state.db).await;
    match result {
        Ok(resp) => {
            let course = Course::find_by_id(resp.last_insert_id)
                .one(&app_state.db)
                .await;
            match course {
                Ok(course) => resp_ok("添加成功", Some(course)),
                Err(_) => resp_ok_err(500, "添加失败"),
            }
        }
        Err(_) => resp_ok_err(500, "添加失败"),
    }
}

// 执行course_del函数
pub async fn course_del(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = params.into_inner();
    let result = Course::delete_by_id(id).exec(&app_state.db).await;
    match result {
        Err(_) => resp_err404("删除失败"),
        Ok(del) => {
            if del.rows_affected == 0 {
                return resp_err404("删除失败");
            } else {
                resp_ok_none("删除成功")
            }
        }
    }
}

// 执行course_update函数
pub async fn course_update(
    app_state: web::Data<AppState>,
    params: web::Json<course::Model>,
) -> Result<HttpResponse, AppError> {
    let db = &app_state.db;
    let params = params.into_inner();
    let pear = Course::find_by_id(params.id).one(db).await;
    match pear {
        Err(_) | Ok(None) => resp_err404("没有找到课程"),
        Ok(course) => {
            let mut course: ActiveModel = course.unwrap().into();
            course.name = Set(params.name.clone());
            course.desc = Set(params.desc.clone());
            let result = course.update(db).await;
            match result {
                Ok(course) => resp_ok("更新成功", Some(course)),
                Err(_) => resp_ok_err(500, "更新失败"),
            }
        }
    }
}

// 执行course_get函数
pub async fn course_get(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = params.into_inner();
    let course = Course::find_by_id(id).one(&app_state.db).await;
    match course {
        Err(_) | Ok(None) => resp_err404("没有找到课程"),
        Ok(course) => resp_ok("查询成功", course),
    }
}

// 执行course_list函数
pub async fn course_list(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let course_list = Course::find().all(&app_state.db).await;
    match course_list {
        Ok(course_list) => resp_ok("查询成功", Some(course_list)),
        Err(_) => resp_err404("没有找到课程"),
    }
}

// 添加测试
#[cfg(test)]
mod tests {
    // use super::*;
    // use actix_web::{http::StatusCode, web};

    // #[actix_rt::test]
    // async fn test_index() {
    //     let resp = index().await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::OK);
    // }

    // #[actix_rt::test]
    // async fn test_health() {
    //     let app_state = web::Data::new(AppState::new());
    //     let resp = health(app_state).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::OK);
    // }

    // #[actix_rt::test]
    // async fn test_course_add() {
    //     let app_state = web::Data::new(AppState::new());
    //     let course = web::Json(Course {
    //         id: None,
    //         name: "rust".to_string(),
    //         desc: "Rust Course List".to_string(),
    //         create_time: None,
    //     });
    //     let resp = course_add(app_state, course).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::OK);
    // }

    // #[actix_rt::test]
    // async fn test_course_del() {
    //     let app_state = web::Data::new(AppState::new());
    //     let course = Course {
    //         id: Some(1),
    //         name: "rust".to_string(),
    //         desc: "Rust Course List".to_string(),
    //         create_time: Some(Utc::now()),
    //     };
    //     app_state.course.lock().unwrap().push(course);
    //     let resp = course_del(app_state, web::Path::from(1)).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::OK);
    // }

    // #[actix_rt::test]
    // async fn test_course_update() {
    //     let app_state = web::Data::new(AppState::new());
    //     let course = Course {
    //         id: Some(1),
    //         name: "rust".to_string(),
    //         desc: "Rust Course List".to_string(),
    //         create_time: Some(Utc::now()),
    //     };
    //     app_state.course.lock().unwrap().push(course);
    //     let course = web::Json(Course {
    //         id: Some(1),
    //         name: "rust".to_string(),
    //         desc: "Rust Course List New".to_string(),
    //         create_time: Some(Utc::now()),
    //     });
    //     let new_app_state = app_state.clone();
    //     let resp = course_update(app_state, course).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::OK);
    //     let course = web::Json(Course {
    //         id: Some(2),
    //         name: "rust".to_string(),
    //         desc: "Rust Course List New".to_string(),
    //         create_time: Some(Utc::now()),
    //     });
    //     let resp = course_update(new_app_state, course).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    // }

    // #[actix_rt::test]
    // async fn test_course_get() {
    //     let app_state = web::Data::new(AppState::new());
    //     let course = Course {
    //         id: Some(1),
    //         name: "rust".to_string(),
    //         desc: "Rust Course List".to_string(),
    //         create_time: Some(Utc::now()),
    //     };
    //     app_state.course.lock().unwrap().push(course);
    //     let new_app_state = app_state.clone();
    //     let resp = course_get(app_state, web::Path::from(1)).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::OK);
    //     let resp = course_get(new_app_state, web::Path::from(2)).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    // }

    // #[actix_rt::test]
    // async fn test_course_list() {
    //     let app_state = web::Data::new(AppState::new());
    //     let app_state1 = app_state.clone();
    //     let app_state2 = app_state.clone();
    //     // 空测试
    //     let resp = course_list(app_state).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    //     let course = Course {
    //         id: Some(1),
    //         name: "rust".to_string(),
    //         desc: "Rust Course List".to_string(),
    //         create_time: Some(Utc::now()),
    //     };
    //     let course1 = course.clone();
    //     let course2 = course.clone();
    //     app_state1.course.lock().unwrap().push(course);
    //     // 测试一个元素
    //     let resp = course_list(app_state1).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::OK);
    //     app_state2.course.lock().unwrap().push(course1);
    //     app_state2.course.lock().unwrap().push(course2);
    //     // 测试多个元素
    //     let resp = course_list(app_state2).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::OK);
    // }
}
