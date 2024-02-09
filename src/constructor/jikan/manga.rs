use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// https://api.jikan.moe/v4/manga

// https://api.jikan.moe/v4/manga/{id}/full

#[derive(Serialize, Deserialize, Clone)]
pub struct Full {
    #[serde(flatten)]
    base: Data,

    relations: Vec<Relation>,

    external: Vec<External>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Search {
    pagination: Pagination,

    data: Vec<Data>,
}

// -------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
struct Data {
    mal_id: i64,

    url: String,

    images: HashMap<String, Image>,

    approved: bool,

    titles: Vec<Title>,

    title: String,

    title_english: String,

    title_japanese: String,

    title_synonyms: Vec<Option<serde_json::Value>>,

    #[serde(rename = "type")]
    data_type: DataType,

    chapters: Option<i64>,

    volumes: Option<i64>,

    status: Status,

    publishing: bool,

    published: Published,

    score: f64,

    scored: f64,

    scored_by: i64,

    rank: i64,

    popularity: i64,

    members: i64,

    favorites: i64,

    synopsis: String,

    background: String,

    authors: Vec<Genre>,

    serializations: Vec<Genre>,

    genres: Vec<Genre>,

    explicit_genres: Vec<Option<serde_json::Value>>,

    themes: Vec<Genre>,

    demographics: Vec<Genre>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Genre {
    mal_id: i64,

    #[serde(rename = "type")]
    genre_type: DataType,

    name: String,

    url: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
enum DataType {
    Anime,
    Manga,
    People,
}

#[derive(Serialize, Deserialize, Clone)]
struct Image {
    image_url: String,

    small_image_url: String,

    large_image_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Published {
    from: String,

    to: Option<String>,

    prop: Prop,

    string: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Prop {
    from: From,

    to: From,
}

#[derive(Serialize, Deserialize, Clone)]
struct From {
    day: Option<i64>,

    month: Option<i64>,

    year: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
enum Status {
    Finished,

    #[serde(rename = "On Hiatus")]
    OnHiatus,

    Publishing,
}

#[derive(Serialize, Deserialize, Clone)]
struct Title {
    #[serde(rename = "type")]
    title_type: TitleType,

    title: String,
}

#[derive(Serialize, Deserialize, Clone)]
enum TitleType {
    Default,

    English,

    French,

    German,

    Japanese,

    Spanish,

    Synonym,
}

// -------------------------------------------------------------------------------------------------
// Manga Full

#[derive(Serialize, Deserialize, Clone)]
struct External {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Relation {
    relation: String,
    entry: Vec<Genre>,
}

// -------------------------------------------------------------------------------------------------
// Manga Search

#[derive(Serialize, Deserialize, Clone)]
struct Pagination {
    last_visible_page: i64,

    has_next_page: bool,

    current_page: i64,

    items: Items,
}

#[derive(Serialize, Deserialize, Clone)]
struct Items {
    count: i64,

    total: i64,

    per_page: i64,
}
