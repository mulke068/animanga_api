use serde::{Deserialize, Serialize};
// use chrono::{DateTime, Utc};
use surrealdb::sql::{Datetime, Thing};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
    pub source: Option<String>, // leer == extern, "upload" == selbst gehostet
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Names {
    pub original: Option<String>,
    pub en: Option<String>,
    pub jp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    // pub id: Option<Thing>, // SurrealDB Thing (e.g. anime:1)
    pub names: Option<Names>,
    pub season: Option<u32>,
    pub episodes: Option<u32>,
    pub score: Option<f32>,
    pub status: Option<String>,
    pub visible: Option<bool>,
    pub description: Option<String>,
    pub types: Option<Vec<String>>,
    pub platforms: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub trailer_urls: Option<Vec<String>>,
    pub info_urls: Option<Vec<String>>,
    pub video_urls: Option<Vec<String>>,
    pub image_urls: Option<Vec<ImageUrl>>,
    pub related: Option<Vec<String>>, // e.g., ["manga:1"]
    // pub created_at: Option<DateTime<Utc>>,
    // pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HasAnime {
    #[serde(rename = "in")]
    pub in_: Thing,   // SurrealDB reserved word "in" => use `#[serde(rename = "in")]`
    pub out: Thing,
    pub watched: Option<u32>,
    pub score: Option<u32>,
    pub status: Option<String>,
    pub favorite: Option<bool>,
    pub notes: Option<String>,
    pub created_at: Option<Datetime>,
    pub updated_at: Option<Datetime>,
}



#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeCreate {
    #[serde(flatten)]
    pub base: Anime,

    pub updated_at: Datetime,
    pub created_at: Datetime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeUpdate {
    #[serde(flatten)]
    pub base: Anime,

    pub updated_at: Datetime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnimeRecord {
    pub id: Thing,

    #[serde(flatten)]
    pub base: Anime,

    pub updated_at: Datetime,
    pub created_at: Datetime,
}

pub trait AnimeField {
    fn base(&self) -> Anime;
}

impl AnimeField for Anime {
    fn base(&self) -> Anime {
        self.clone()
    }
}