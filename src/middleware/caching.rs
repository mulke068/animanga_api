use actix_web::web;
use redis::{Commands, RedisError};
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
    state: web::Data<AppData>,
}

impl<'de> serde::Deserialize<'de> for AppData {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Implement the deserialization logic here
        unimplemented!()
    }
}

// Remove the implementation of Deserialize for &'a web::Data<AppData>

impl Serialize for AppData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Implement the serialization logic here
        // Return the serialization result with the expected types
        serializer.serialize_unit()
    }
}

#[allow(dead_code)]
impl<'a> Caching {
    /// ## Create a new construct

    pub fn new(uri: String, state: &'a web::Data<AppData> ) -> Self {
        Self { uri, time: 3600 , state: state.clone()}
    }

    /// ## Create the key from the Struct

    fn get_key(&self) -> String {
        // format!("{}:{}", self.method, self.uri)
        format!("{}", self.uri)
    }
    
    /// ## Checks the connection to the cache
    
    fn connection(&self) -> Result<redis::Connection, RedisError> {
        self.state.redis.get_connection()
    }

    /// ## Gets the Data from the cache

    pub async fn get(&self) -> Option<String> {
        if let Ok(mut connection) = self.connection() {
            return connection.get(self.get_key()).ok();
        } else {
            return None;
        }
    }

    /// ## Saves the req with the uri as key in the cache
    /// Can also be used for update the query

    pub async fn set(&self, req: &String) {
        if let Ok(mut connection) = self.connection() {
            return connection.set_ex(self.get_key(), req, self.time.try_into().unwrap()).unwrap();
        } else {
            return;
        }
    }

    /// ## Delete the cached Data

    pub async fn delete(&self) {
        if let Ok(mut connection) = self.connection() {
            return connection.del(self.get_key()).unwrap();
        } else {
            return;
        }
    }

    /// ## Gives the Timer time

    pub async fn timer_get(&self) -> isize {
        if let Ok(mut connection) = self.connection() {
            return connection.ttl(self.get_key()).unwrap();
        } else {
            return 0;
        }
    }

    /// ## Resets the Timer to the given default time

    pub async fn timer_reset(&self) -> isize {
        if let Ok(mut connection) = self.connection() {
            return connection.expire(self.get_key(), self.time.try_into().unwrap()).unwrap();
        } else {
            return 0;
        }
    }
}
