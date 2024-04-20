use std::char;

use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use rand::{distributions::Alphanumeric, Rng};
use surrealdb::sql::{Datetime, Thing};

use crate::AppServices;
use serde::{Deserialize, Serialize};

use super::user_structs::UsersRecord;

pub trait AccountField {
    fn base(&self) -> Account;
    async fn find_uid(&self, service: web::Data<AppServices>) -> String;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenRecord {
    pub token: String,
    pub out: Thing,
    pub created_at: Datetime,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub token: String,
    pub out: Thing,
    pub created_at: Datetime,
}

impl AccountField for Account {
    fn base(&self) -> Account {
        self.clone()
    }

    async fn find_uid(&self, service: web::Data<AppServices>) -> String {
        let username = self.username.clone();
        let password = self.password.clone();

        let query = "SELECT * FROM user WHERE username = $user AND crypto::argon2::compare(password, $pass)";

        let record: Option<UsersRecord> = service
            .surreal
            .query(query)
            .bind(("user", &username))
            .bind(("pass", &password))
            .await
            .unwrap()
            .take(0)
            .unwrap();

        // log::info!("Record: {:?}", record);

        record.unwrap().id.id.to_string()
    }
}

async fn find_user(uid: &str, service: web::Data<AppServices>) -> bool {
    let record: Option<UsersRecord> = service.surreal.select(("user", uid)).await.unwrap();

    record.is_some()
}

async fn find_token(token: &str, service: web::Data<AppServices>) -> String {
    let query = format!("SELECT * FROM token WHERE token = {}", &token.to_string());

    let record: Vec<TokenRecord> = match service.surreal.query(&query).await {
        Ok(mut data) => data.take(0).unwrap(),
        Err(e) => {
            eprintln!("Surrealdb Query Problem {}", e);
            Vec::new()
        }
    };

    let res = match &record.len() {
        1 => {
            let data = &record[1].out.id;
            serde_json::to_string(&data).unwrap()
        }
        _ => {
            eprintln!("No Data Found");
            "Data Not Found".to_string()
        }
    };

    res
}

async fn create_token(uid: &str, service: web::Data<AppServices>) -> String {
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let record: Vec<TokenRecord> = match service
        .surreal
        .create("token")
        .content(Token {
            token,
            out: Thing {
                tb: ("user".to_string()),
                id: (uid.into()),
            },
            created_at: Datetime::default(),
        })
        .await
    {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error At Surrealdb: {:?}", e);
            Vec::new()
        }
    };

    let res: String = match &record.len() {
        1 => {
            let data = &record[0].token;
            serde_json::to_string(&data).unwrap()
        }
        _ => String::from("Data Not Found"),
    };

    res
}

#[derive(Debug, Serialize, Deserialize)]
struct QToken {
    token: String,
}

pub async fn handler_user_auth_get(
    params: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<QToken>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed to query params"));

    // log::info!("Token: {}", param.token);
    //
    let uid: String = find_token(&param.token, service.clone()).await;

    match find_user(&uid, service.clone()).await {
        true => HttpResponse::Ok().json(&uid),
        false => HttpResponse::NotFound().body("Your token is invalid"),
    }
}

pub async fn handler_user_login_post(
    params: web::Json<Account>,
    service: web::Data<AppServices>,
) -> impl Responder {
    let account = params.into_inner();

    let uid = account.find_uid(service.clone()).await;
    let token = create_token(&uid, service.clone()).await;

    let result = format!("{{\"token\": \"{:?}\", \"uid\": {:?}}}", &token, &uid);
    HttpResponse::Ok().body(result)
}
