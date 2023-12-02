use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Anime9 {
    id: String,
    title: String,
    url: String,

    #[serde(rename = "releaseDate")]
    release_data: String,
    description: String,

    genres: Vec<Option<serde_json::Value>>,

    #[serde(rename = "subOrDub")]
    sub_or_dub: String,

    #[serde(rename = "type")]
    anie9_type: String,
    status: String,

    #[serde(rename = "otherName")]
    other_name: String,

    #[serde(rename = "totalEpisodes")]
    total_episodes: i64,
    episodes: Vec<Episode>,
}

#[derive(Serialize, Deserialize)]
struct Episode {
    id: String,
    number: i64,
    url: String,
}
