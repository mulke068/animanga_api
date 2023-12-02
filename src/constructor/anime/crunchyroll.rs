use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Crunchyroll {
    id: String,
    title: String,

    #[serde(rename = "isAdult")]
    is_adult: bool,
    image: String,
    cover: String,
    description: String,

    #[serde(rename = "releaseDate")]
    release_date: i64,

    gernes: Vec<Option<serde_json::Value>>,
    season: String,

    #[serde(rename = "hasDub")]
    has_dub: bool,

    #[serde(rename = "hasSub")]
    has_sub: bool,
    rating: String,

    #[serde(rename = "ratingTotal")]
    rating_total: i64,

    recommendations: Vec<Recommendation>,
    episodes: Vec<Episode>,
}

#[derive(Serialize, Deserialize)]
struct Recommendation {
    id: String,
    title: String,
    image: String,
    cover: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Episode {
    #[serde(rename = "seasonName")]
    season_name: Vec<SeasonName>,
}

#[derive(Serialize, Deserialize)]
struct SeasonName {
    id: String,
    season_number: i64,
    episode_number: i64,
    title: String,
    image: String,
    description: String,

    #[serde(rename = "releaseDate")]
    release_date: String,

    #[serde(rename = "isHD")]
    is_hd: bool,

    #[serde(rename = "isAdult")]
    is_adult: bool,

    #[serde(rename = "isDubbed")]
    is_dubbed: bool,

    #[serde(rename = "isSubbed")]
    is_subbed: bool,
}
