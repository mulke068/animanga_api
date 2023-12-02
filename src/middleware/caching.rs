use actix_web::web;
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::AppData;

/// Test
/// ```
/// use api::middleware::caching::Caching;
///
/// let data = Caching {
///     uri: String::from("Test Url")
/// };
///
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Caching {
    // pub method: String,
    pub uri: String,
    pub time: usize,
}

#[allow(dead_code)]
impl Caching {
    /// ## Create a new construct

    pub fn new(uri: String) -> Self {
        Self { uri, time: 3600 }
    }

    /// ## Create the key from the Struct

    fn get_key(&self) -> String {
        // format!("{}:{}", self.method, self.uri)
        format!("{}", self.uri)
    }

    /// ## Gets the Data from the cache

    pub async fn get(&self, state: &web::Data<AppData>) -> Option<String> {
        state
            .redis
            .get_connection()
            .unwrap()
            .get(self.get_key())
            .unwrap()
    }

    /// ## Saves the req with the uri as key in the cache
    /// Can also be used for update the query

    pub async fn set(&self, req: &String, state: &web::Data<AppData>) {
        state
            .redis
            .get_connection()
            .unwrap()
            .set_ex(self.get_key(), req, self.time)
            .unwrap()
    }

    /// ## Delete the cached Data

    pub async fn delete(&self, state: &web::Data<AppData>) {
        state
            .redis
            .get_connection()
            .unwrap()
            .del(self.get_key())
            .unwrap()
    }

    /// ## Gives the Timer time

    pub async fn timer_get(&self, state: &web::Data<AppData>) -> isize {
        state
            .redis
            .get_connection()
            .unwrap()
            .ttl(self.get_key())
            .unwrap()
    }

    /// ## Resets the Timer to the given default time

    pub async fn timer_reset(&self, state: &web::Data<AppData>) -> isize {
        state
            .redis
            .get_connection()
            .unwrap()
            .expire(self.get_key(), self.time)
            .unwrap()
    }
}
