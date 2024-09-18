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
    fn authenticate_user(
        &self,
        service: web::Data<AppServices>,
    ) -> impl std::future::Future<Output = String> + Send;
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

    // Authenticate User
    // If the user is not found, it will return an empty string
    // If the user is found, it will return the user id
    // @return String
    #[inline]
    async fn authenticate_user(&self, service: web::Data<AppServices>) -> String /* GET USER ID */ {
        let username = self.username.clone();
        let password = self.password.clone();

        let query = "SELECT * FROM user WHERE username = $user AND crypto::argon2::compare(password, $pass)";

        let record: Option<UsersRecord> = service
            .surreal
            .query(query)
            .bind(("user", username))
            .bind(("pass", password))
            .await
            .unwrap()
            .take(0)
            .unwrap();

        // log::info!("Record: {:?}", record);

        record.unwrap().id.id.to_string()
    }
}

async fn find_token_or_uid(search: &str, service: web::Data<AppServices>) -> String {
    let query = format!("SELECT * FROM token WHERE {:?}", &search.to_string());

    // log::info!("Query: {}", &query);

    let record: Vec<TokenRecord> = match service.surreal.query(&query).await {
        Ok(mut data) => data.take(0).unwrap(),
        Err(e) => {
            log::error!("Error At Surrealdb: {:?}", e);
            Vec::new()
        }
    };

    // log::info!("Record: {:?}", &record);

    let res = match &record.len() {
        1 => {
            let uid = &record[0].out.id.to_string();
            let token = &record[0].token;
            format!("{{\"token\": {:?}, \"uid\": {:?}}}", &token, &uid)
            // serde_json::to_string(&data).unwrap()
        }
        _ => {
            log::error!("No Token Found");
            "Token Not Found".to_string()
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
            out: Thing::from(("user", uid)),
            created_at: Datetime::default(),
        })
        .await
    {
        Ok(data) => data.unwrap_or(Vec::new()),
        Err(_e) => {
            // log::error!("Error At Surrealdb: {:?}", e);
            Vec::new()
        }
    };

    let res = match &record.len() {
        1 => {
            let data = &record[0].token;
            data.to_string()
            // serde_json::to_string(&data).unwrap()
        }
        _ => String::from("Error Creating Token"),
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

    //log::info!("Token: {}", param.token);

    match Some(find_token_or_uid(&param.token, service.clone()).await) {
        Some(u_token) => HttpResponse::Found().body(u_token),
        None => HttpResponse::NotFound().body("Your token is invalid"),
    }
}

pub async fn handler_user_login_post(
    params: web::Json<Account>,
    service: web::Data<AppServices>,
) -> impl Responder {
    let account = params.into_inner();

    //log::info!("Account: {:?}", &account);

    let uid = account.authenticate_user(service.clone()).await;

    //log::info!("UID: {:?}", &uid);

    let find_user_token: String = find_token_or_uid(&uid, service.clone()).await;

    //log::info!("Token: {:?}", &token);

    if &find_user_token != "Token Not Found" {

        return HttpResponse::Found().body(find_user_token);

    } else if !&find_user_token.is_empty() {

        let token = create_token(&uid, service.clone()).await;

        let result = format!("{{\"token\": \"{}\", \"uid\": {:?}}}", &token, &uid);
        return HttpResponse::Created().body(result)

    } else {
        return HttpResponse::NotFound().body("Your token is invalid");
    }
}
