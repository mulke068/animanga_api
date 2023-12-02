use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Anime {
    mal_id: i64,

    url: String,

    images: HashMap<String, Image>,

    trailer: Trailer,

    approved: bool,

    titles: Vec<Title>,

    title: String,

    title_english: String,

    title_japanese: String,

    title_synonyms: Vec<Option<serde_json::Value>>,

    #[serde(rename = "type")]
    data_type: String,

    source: String,

    episodes: i64,

    status: String,

    airing: bool,
    
    aired: Aired,

    duration: String,

    rating: String,

    score: f64,

    scored_by: i64,

    rank: i64,

    popularity: i64,

    members: i64,

    favorites: i64,

    synopsis: String,

    background: String,

    season: String,

    year: i64,

    broadcast: Broadcast,

    producers: Vec<Genre>,

    licensors: Vec<Genre>,

    studios: Vec<Genre>,

    genres: Vec<Genre>,

    explicit_genres: Vec<Option<serde_json::Value>>,

    themes: Vec<Genre>,

    demographics: Vec<Option<serde_json::Value>>,

    relations: Vec<Relation>,

    theme: Theme,

    external: Vec<External>,

    streaming: Vec<External>,
}

#[derive(Serialize, Deserialize)]
pub struct Aired {
    from: String,

    to: String,

    prop: Prop,

    string: String,
}

#[derive(Serialize, Deserialize)]
pub struct Prop {
    from: From,

    to: From,
}

#[derive(Serialize, Deserialize)]
pub struct From {
    day: i64,

    month: i64,

    year: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Broadcast {
    day: String,

    time: String,

    timezone: String,

    string: String,
}

#[derive(Serialize, Deserialize)]
pub struct External {
    name: String,

    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Genre {
    mal_id: i64,

    #[serde(rename = "type")]
    genre_type: Type,

    name: String,

    url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Anime,

    Manga,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    image_url: String,

    small_image_url: String,

    large_image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Relation {
    relation: String,

    entry: Vec<Genre>,
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    openings: Vec<String>,

    endings: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "type")]
    title_type: String,

    title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Trailer {
    youtube_id: String,

    url: String,

    embed_url: String,

    images: Images,
}

#[derive(Serialize, Deserialize)]
pub struct Images {
    image_url: String,

    small_image_url: String,

    medium_image_url: String,

    large_image_url: String,

    maximum_image_url: String,
}
