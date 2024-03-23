
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};

use crate::AppServices;
use serde::{Deserialize, Serialize};


use futures::future::Future;

use super::user_structs::UsersRecord;

pub trait AccountField {
    fn base(&self) -> Account;
    fn find_uid(&self, service: web::Data<AppServices>) -> impl Future<Output = String> + Send + 'static;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    #[serde(rename = "user")]
    pub username: String,
    #[serde(rename = "pass")]
    pub password: String,
}

impl AccountField for Account {
    fn base(&self) -> Account {
        self.clone()
    }

    fn find_uid(&self, service: web::Data<AppServices>) -> impl Future<Output = String> + Send + 'static {
        let username = self.username.clone();
        let password = self.password.clone();

        async move {
            let query = "SELECT * FROM user WHERE username = $user AND crypto::argon2::compare(password, $pass)";

            let record: Option<UsersRecord> = service.surreal.query(query)
                .bind(("user", &username))
                .bind(("pass", &password))
                .await.unwrap()
                .take(0).unwrap();

            // log::info!("Record: {:?}", record);

            return record.unwrap().id.id.to_string();
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String,
}

pub async fn handler_user_auth_get(
    params: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<Token>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed to query params"));

    log::info!("Token: {}", param.token);

    match service.surreal.authenticate(param.token.clone()).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::NotFound().body("Your token is invalid"),
    }
}

pub mod login {
    use actix_web::{web, HttpResponse, Responder};
    use surrealdb::opt::auth::Scope;

    use crate::AppServices;

    use super::{Account, AccountField};

    pub async fn handler_user_login_post(
        params: web::Json<Account>,
        service: web::Data<AppServices>,
    ) -> impl Responder {
        let token: String = match service
            .surreal
            .signin(Scope {
                namespace: "test",
                database: "test",
                scope: "account",
                params: Account {
                    username: params.username.clone(),
                    password: params.password.clone(),
                },
            })
            .await
        {
            Ok(data) => data.into_insecure_token(),
            Err(e) => format!("Error Occured {}", e), // Fix: Return a String instead of an HttpResponse
        };

        let uid = Account::find_uid(&params, service.clone()).await;

        let result = format!("{{\"token\": \"{}\", \"uid\": {:?}}}", token, uid);
        HttpResponse::Ok().body(result)
    }
}
