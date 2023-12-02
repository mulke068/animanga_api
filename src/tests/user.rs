use crate::handlers::user::main::*;
use actix_web::{test, web, App};
use reqwest::StatusCode;
use serde_json::json;

#[actix_web::test]
async fn test_create() {
    let mut app =
        test::init_service(App::new().route("/user", web::post().to(handler_user_post))).await;

    let req = test::TestRequest::post()
        .uri("/user")
        .set_json(&json!({
            "username": "test_user",
            "password": "test_password",
            "email": "test_email",
            "name": {
                "first": "test_first_name",
                "last": "test_last_name",
            },
            "status": true,
            "role": "test_role",
            "permission": {
                "grade": 1,
                "create": true,
                "read": true,
                "update": true,
                "delete": true,
            },
        }))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    let body: UsersRecord = test::try_read_body_json(resp).await.unwrap();
    assert_eq!(body.base.name.first, "test_first_name");
    assert_eq!(body.base.password, "test_password");
    assert_eq!(body.base.permission.grade, 1);

    let _id = body.id.id.to_string();
}

#[actix_web::test]
async fn test_get() {
    let mut app =
        test::init_service(App::new().route("/user", web::get().to(handler_user_get))).await;

    let req = test::TestRequest::get().uri("/user?uid=1").to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_update() {
    let mut app =
        test::init_service(App::new().route("/user?id=1", web::get().to(handler_user_patch))).await;

    let req = test::TestRequest::patch()
        .uri("/user?id=1")
        .set_json(&json!({
            "username": "test_user",
            "password": "test_password",
            "email": "test_email",
            "name": {
                "first": "test_first_name",
                "last": "test_last_name",
            },
            "status": true,
            "role": "test_role",
            "permission": {
                "grade": 1,
                "create": true,
                "read": true,
                "update": true,
                "delete": true,
            },
        }))
        .to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: UsersRecord = test::try_read_body_json(resp).await.unwrap();

    assert_eq!(body.base.name.first, "test_first_name");
    assert_eq!(body.base.password, "test_password");
    assert_eq!(body.base.permission.grade, 1);
}
