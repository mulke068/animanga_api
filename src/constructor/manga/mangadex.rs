use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Mangadex {
    id: String,
    title: String,

    #[serde(rename = "altTitles")]
    alt_titles: Vec<Option<serde_json::Value>>,

    genres: Vec<Option<serde_json::Value>>,

    #[serde(rename = "headerForImage")]
    header_for_image: String,

    image: String,

    chapters: Vec<Chapter>,
}

#[derive(Serialize, Deserialize)]
struct Chapter {
    id: String,
    title: String,

    #[serde(rename = "releaseDate")]
    release_date: String,
}
