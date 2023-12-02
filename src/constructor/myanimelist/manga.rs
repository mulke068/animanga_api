use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Manga {
    mal_id: i64,

    url: String,

    images: HashMap<String, Images>,

    approved: bool,

    titles: Vec<Titles>,

    title: String,

    title_english: String,

    title_japanese: String,

    title_synonyms: Vec<Option<serde_json::Value>>,

    #[serde(rename = "type")]
    data_type: String,

    chapters: i64,

    volumes: i64,

    status: String,

    publishing: bool,

    published: Published,

    score: f64,

    scored: i64,

    scored_by: i64,

    rank: i64,

    popularity: i64,

    members: i64,

    favorites: i64,

    synopsis: String,

    background: String,

    authors: Vec<Author>,

    serializations: Vec<Genre>,

    genres: Vec<Genre>,

    explicit_genres: Vec<Option<serde_json::Value>>,

    themes: Theme,

    demographics: Vec<Option<serde_json::Value>>,

    relations: Vec<Relation>,

    external: Vec<External>,
}

#[derive(Serialize, Deserialize)]
struct Images {
    image_url: String,
    small_image_url: String,
    large_image_url: String,
}

#[derive(Serialize, Deserialize)]
struct Titles {
    #[serde(rename = "type")]
    titles_type: String,
    title: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Type {
    Anime,
    Manga,
}

#[derive(Serialize, Deserialize)]
struct Published {
    from: String,
    to: String,
    prop: Prop,
    string: String,
}

#[derive(Serialize, Deserialize)]
struct Prop {
    from: From,
    to: From,
}

#[derive(Serialize, Deserialize)]
struct From {
    day: i64,
    month: i64,
    year: i64,
}

#[derive(Serialize, Deserialize)]
struct Author {
    mal_id: i64,
    #[serde(rename = "type")]
    data_type: Type,
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Genre {
    mal_id: i64,
    #[serde(rename = "type")]
    data_type: Type,
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Relation {
    relation: String,
    entry: Vec<Genre>,
}

#[derive(Serialize, Deserialize)]
struct Theme {
    openings: Vec<String>,
    endings: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct External {
    name: String,
    url: String,
}
