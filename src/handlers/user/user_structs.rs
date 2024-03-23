use regex::Regex;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPermission {
    pub grade: u8,
    pub read: bool,
    pub write: bool,
    pub update: bool,
    pub delete: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserName {
    pub first: String,
    pub last: String,
}

pub trait UsersField {
    fn base(&self) -> Users;
    fn create(&mut self) -> UsersCreate;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Users {
    pub username: String,
    pub name: UserName,
    pub email: String,
    pub password: String,
    pub status: bool,
    pub role: Option<String>,
    pub permission: Option<UserPermission>,
}

impl UsersField for Users {
    fn base(&self) -> Users {
        self.clone()
    }

    fn create(&mut self) -> UsersCreate {
        // Checking Email if the email pattern is valid
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !re.is_match(&self.email) {
            panic!("Invalid Email");
        };

        if self.role.is_none() {
            self.role = Some("user".to_string());
        }

        if self.permission.is_none() {
            self.permission = Some(UserPermission {
                grade: 254,
                read: true,
                write: false,
                update: false,
                delete: false,
            });
        }


        UsersCreate {
            base: Users {
                username: self.username.clone(),
                name: self.name.clone(),
                email: self.email.clone(),
                password: self.password.clone(),
                status: self.status.clone(),
                role: self.role.clone(),
                permission: self.permission.clone(),
            },
            updated_at: Datetime::default(),
            created_at: Datetime::default(),
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersCreate {
    #[serde(flatten)]
    pub base: Users,
    pub updated_at: Datetime,
    pub created_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersUpdate {
    #[serde(flatten)]
    pub base: Users,
    pub updated_at: Datetime,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UsersRecord {
    pub id: Thing,
    #[serde(flatten)]
    pub base: Users,
    pub updated_at: Datetime,
    pub created_at: Datetime,
}
