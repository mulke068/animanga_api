use std::collections::HashMap;

use serde::{Deserialize, Serialize };

// https://api.jikan.moe/v4/anime

// https://api.jikan.moe/v4/anime/{id}/full

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Full {
    #[serde(flatten)]
    pub base: Data,

    pub relations: Vec<Relation>,

    pub theme: Theme,

    pub external: Vec<External>,

    pub streaming: Vec<External>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Search {
    pub pagination: Pagination,

    pub data: Vec<Data>,
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    pub mal_id: i64,

    pub url: String,

    pub images: HashMap<String, Image>,

    pub trailer: Trailer,

    pub approved: bool,

    pub titles: Vec<Title>,

    pub title: Option<String>,

    pub title_english: Option<String>,

    pub title_japanese: Option<String>,

    pub title_synonyms: Vec<Option<serde_json::Value>>,

    #[serde(rename = "type")]
    pub data_type: DataType,

    pub source: Source,

    pub episodes: Option<i64>,

    pub status: Status,

    pub airing: bool,

    pub aired: Aired,

    pub duration: String,

    pub rating: Rating,

    pub score: Option<f64>,

    pub scored_by: i64,

    pub rank: Option<i64>,

    pub popularity: i64,

    pub members: i64,

    pub favorites: i64,

    pub synopsis: Option<String>,

    pub background: Option<String>,

    pub season: Option<String>,

    pub year: Option<i64>,

    pub broadcast: Broadcast,

    pub producers: Vec<Genre>,

    pub licensors: Vec<Genre>,

    pub studios: Vec<Genre>,

    pub genres: Vec<Genre>,

    pub explicit_genres: Vec<Option<serde_json::Value>>,

    pub themes: Vec<Genre>,

    pub demographics: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aired {
    pub from: String,

    pub to: Option<String>,

    pub prop: Prop,

    pub string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prop {
    pub from: From,

    pub to: From,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct From {
    pub day: Option<i64>,

    pub month: Option<i64>,

    pub year: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Broadcast {
    pub day: Option<String>,

    pub time: Option<String>,

    pub timezone: Option<String>,

    pub string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataType {
    #[serde(rename = "ONA")]
    Ona,

    #[serde(rename = "OVA")]
    Ova,

    Special,

    #[serde(rename = "TV")]
    Tv,
    
    Movie,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Genre {
    pub mal_id: i64,

    #[serde(rename = "type")]
    pub genre_type: GenreType,

    pub name: String,

    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GenreType {
    Anime,

    Manga,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub image_url: String,

    pub small_image_url: String,

    pub large_image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Rating {
    #[serde(rename = "PG-13 - Teens 13 or older")]
    Pg13Teens13OrOlder,

    #[serde(rename = "PG - Children")]
    PgChildren,

    #[serde(rename = "R - 17+ (violence & profanity)")]
    R17ViolenceProfanity,

    #[serde(rename = "R+ - Mild Nudity")]
    RMildNudity,

    #[serde(rename = "Rx - Hentai")]
    RxHentai,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Source {
    #[serde(rename = "Light novel")]
    LightNovel,

    Manga,

    Original,

    #[serde(rename = "Visual novel")]
    VisualNovel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    #[serde(rename = "Currently Airing")]
    CurrentlyAiring,

    #[serde(rename = "Finished Airing")]
    FinishedAiring,

    #[serde(rename = "Not yet aired")]
    NotYetAired,

    Cancelled,

    Hiatus,

    TBA,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Title {
    #[serde(rename = "type")]
    pub title_type: TitleType,

    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TitleType {
    Default,

    English,

    French,

    German,

    Japanese,

    Spanish,

    Synonym,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trailer {
    pub youtube_id: Option<String>,

    pub url: Option<String>,

    pub embed_url: Option<String>,

    pub images: Images,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    pub image_url: Option<String>,

    pub small_image_url: Option<String>,

    pub medium_image_url: Option<String>,

    pub large_image_url: Option<String>,

    pub maximum_image_url: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// Anime Full

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct External {
    pub name: String,

    puburl: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Relation {
    pub relation: String,

    pubentry: Vec<Genre>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Theme {
    pub openings: Vec<String>,

    pubendings: Vec<String>,
}

// -------------------------------------------------------------------------------------------------
// Anime Search

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pagination {
    pub last_visible_page: i64,

    pub has_next_page: bool,

    pub current_page: i64,

    pub items: Items,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Items {
    pub count: i64,

    pub total: i64,

    pub per_page: i64,
}
