#![recursion_limit = "256"]

pub mod constructor;
pub mod handlers;
pub mod middleware;
pub mod modules;
pub mod tests;

/// # Where the DB client lives
///
///
/// use crate::AppServices;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AppServices {
    pub surreal: surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
    pub redis: redis::Client,
    pub meilisearch: meilisearch_sdk::Client,
}

/// ## Split String at char
/// return splitted String as String
/// ```
///  use api::handlers::splitted_data_at;
///
///  let data = "anime:1234".to_string();
///  let data_new = splitted_data_at(data, ":");
///  debug_assert_eq!(data_new, "1234");
/// ```
pub fn splitted_data_at(data: String, split_chars: &str) -> String {
    let data_len = data.find(split_chars).unwrap();
    let (_, data_new) = data.split_at(data_len + 1);
    return data_new.to_string();
}
