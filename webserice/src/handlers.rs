use super::apperror::AppError;
use super::models::*;
use super::state::*;
use actix_web::{web, HttpResponse};
use chrono::Utc;

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
    course: web::Json<Course>,
) -> Result<HttpResponse, AppError> {
    let mut course = course.into_inner();
    let mut course_list = app_state.course.lock().unwrap();
    let course_count = course_list.clone().into_iter().count();
    course.id = Some(course_count + 1);
    course.create_time = Some(Utc::now());
    let resp = course.clone();
    course_list.push(course);
    resp_ok("添加成功", Some(resp))
}

// 执行course_del函数
pub async fn course_del(
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, AppError> {
    let mut course_list = app_state.course.lock().unwrap();
    let id = params.into_inner();
    let idx = course_list.partition_point(|x| x.id.unwrap() == id);
    match idx {
        0 => return resp_err404("没有找到课程"),
        _ => {
            course_list.remove(idx - 1);
            resp_ok_none("删除成功")
        }
    }
}

// 执行course_update函数
pub async fn course_update(
    app_state: web::Data<AppState>,
    params: web::Json<Course>,
) -> Result<HttpResponse, AppError> {
    let mut course_list = app_state.course.lock().unwrap();
    let course = params.into_inner();
    let id = course.id.unwrap();
    let idx = course_list.partition_point(|x| x.id.unwrap() == id);
    if idx == 0 {
        resp_err404("没有找到课程")
    } else {
        let resp = course.clone();
        course_list.push(course);
        resp_ok("更新成功", Some(resp))
    }
}

// 执行course_get函数
pub async fn course_get(
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> Result<HttpResponse, AppError> {
    let course_list = app_state.course.lock().unwrap();
    let id = params.into_inner();
    let resp = course_list
        .clone()
        .into_iter()
        .find(|x| x.id.unwrap() == id);
    match resp {
        Some(resp) => resp_ok("更新成功", Some(resp)),
        None => resp_err404("没有找到课程"),
    }
}

// 执行course_list函数
pub async fn course_list(app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let course_list = app_state.course.lock().unwrap();
    if course_list.len() == 0 {
        return resp_err404("没有找到课程");
    } else {
        resp_ok("查询成功", Some(course_list.clone()))
    }
}

// 添加测试
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, web};

    #[actix_rt::test]
    async fn test_index() {
        let resp = index().await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_health() {
        let app_state = web::Data::new(AppState::new());
        let resp = health(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_course_add() {
        let app_state = web::Data::new(AppState::new());
        let course = web::Json(Course {
            id: None,
            name: "rust".to_string(),
            desc: "Rust Course List".to_string(),
            create_time: None,
        });
        let resp = course_add(app_state, course).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_course_del() {
        let app_state = web::Data::new(AppState::new());
        let course = Course {
            id: Some(1),
            name: "rust".to_string(),
            desc: "Rust Course List".to_string(),
            create_time: Some(Utc::now()),
        };
        app_state.course.lock().unwrap().push(course);
        let resp = course_del(app_state, web::Path::from(1)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_course_update() {
        let app_state = web::Data::new(AppState::new());
        let course = Course {
            id: Some(1),
            name: "rust".to_string(),
            desc: "Rust Course List".to_string(),
            create_time: Some(Utc::now()),
        };
        app_state.course.lock().unwrap().push(course);
        let course = web::Json(Course {
            id: Some(1),
            name: "rust".to_string(),
            desc: "Rust Course List New".to_string(),
            create_time: Some(Utc::now()),
        });
        let new_app_state = app_state.clone();
        let resp = course_update(app_state, course).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let course = web::Json(Course {
            id: Some(2),
            name: "rust".to_string(),
            desc: "Rust Course List New".to_string(),
            create_time: Some(Utc::now()),
        });
        let resp = course_update(new_app_state, course).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_course_get() {
        let app_state = web::Data::new(AppState::new());
        let course = Course {
            id: Some(1),
            name: "rust".to_string(),
            desc: "Rust Course List".to_string(),
            create_time: Some(Utc::now()),
        };
        app_state.course.lock().unwrap().push(course);
        let new_app_state = app_state.clone();
        let resp = course_get(app_state, web::Path::from(1)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = course_get(new_app_state, web::Path::from(2)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_course_list() {
        let app_state = web::Data::new(AppState::new());
        let app_state1 = app_state.clone();
        let app_state2 = app_state.clone();
        // 空测试
        let resp = course_list(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let course = Course {
            id: Some(1),
            name: "rust".to_string(),
            desc: "Rust Course List".to_string(),
            create_time: Some(Utc::now()),
        };
        let course1 = course.clone();
        let course2 = course.clone();
        app_state1.course.lock().unwrap().push(course);
        // 测试一个元素
        let resp = course_list(app_state1).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        app_state2.course.lock().unwrap().push(course1);
        app_state2.course.lock().unwrap().push(course2);
        // 测试多个元素
        let resp = course_list(app_state2).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}