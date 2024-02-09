#![allow(dead_code)]

// use crate::constructor::jikan::search::anime::Anime;

use crate::constructor::jikan::anime::Search as AnimeSearch;
use crate::constructor::jikan::anime::Status;
use crate::handlers::anime::main::{Anime, AnimeNames};
use crate::modules::error_handler::CustomError;

pub(super) async fn module_request_jikan(req: String) -> Result<String, CustomError> {
    let time_start = std::time::Instant::now();
    let client = reqwest::Client::new();

    let url = format!("https://api.jikan.moe/v4/anime?q={}", req);

    {
        log::info!("Request URL: {:?}", url);
    }

    let body = client
        .get(url)
        .send()
        .await
        .map_err(|e| CustomError::RequestError(e.to_string()))?
        .text()
        .await
        .map_err(|e| CustomError::ParseError(e.to_string()))?;

    {
        log::info!("Request Received");
        log::info!("Request Time: {:?}", time_start.elapsed());
    }

    let req: AnimeSearch =
        serde_json::from_str(&body).map_err(|e| CustomError::ConstructError(e.to_string()))?;

    {
        log::info!("Request Parsed");
        log::info!("Parsing Time: {:?}", time_start.elapsed());
    }

    {
        for (k, v) in req.data.iter().enumerate() {
            log::info!("[{:?}] Anime ID: {:?}", k, v.mal_id);
        }
    }

    let converted = convert_search_to_anime(&req);

    {
        log::info!("Request Converted");
        log::info!("Conversion Time: {:?}", time_start.elapsed());
    }

    let res = serde_json::to_string(&converted).unwrap();
    // let res = serde_json::to_string(&req).unwrap();

    {
        log::info!("Request Serialized");
        log::info!("Serialized Time: {:?}", time_start.elapsed());
    }

    Ok(res)
}

fn convert_search_to_anime(search: &AnimeSearch) -> Vec<Anime> {
    search
        .data
        .iter()
        .map(|data| {
            let names = AnimeNames {
                original: data.title.clone().unwrap_or_default(),
                en: Some(data.title_english.clone().unwrap_or_default()),
                jp: Some(data.title_japanese.clone().unwrap_or_default()),
            };

            let status = match data.status {
                Status::FinishedAiring => "Finished Airing".to_string(),
                Status::CurrentlyAiring => "Currently Airing".to_string(),
                Status::NotYetAired => "Not Yet Aired".to_string(),
                Status::Cancelled => "Cancelled".to_string(),
                Status::Hiatus => "Hiatus".to_string(),
                Status::TBA => "TBA".to_string(),
            };

            let mut types = vec![];
            // if let Some(data_type) = &data.data_type {
            //     types.push(format!("{:?}", *data_type));
            // }
            types.push(format!("{:?}", &data.data_type));

            let mut platforms = vec![];
            // if let Some(source) = data.source.clone() {
            //     let source_str = match source {
            //         // Source::TV => "TV",
            //         // Source::OVA => "OVA",
            //         // Source::Movie => "Movie",
            //         // Source::Special => "Special",
            //         // Source::ONA => "ONA",
            //         // Source::Music => "Music",
            //         Source::Manga => "Manga",
            //         Source::LightNovel => "Light Novel",
            //         Source::Original => "Original",
            //         Source::VisualNovel => "Visual Novel",
            //         // Source::Novel => "Novel",
            //         // Source::OneShot => "OneShot",
            //         // Source::Doujinshi => "Doujinshi",
            //         // Source::Unknown => "Unknown",
            //     };
            //     platforms.push(source_str.to_string());
            // }
            platforms.push(format!("{:?}", &data.source));

            let genres = data.genres.iter().map(|genre| genre.name.clone()).collect();

            // let tags = data
            //     .themes
            //     .iter()
            //     .map(|genre| genre.name.clone())
            //     .chain(data.demographics.iter().map(|genre| genre.as_ref().unwrap_or(&serde_json::Value::Null).to_string()))
            //     .collect();
            let tags = data
                .themes
                .iter()
                .map(|genres| genres.name.clone())
                .collect();

            let trailer_urls = data
                .trailer
                .url
                .as_ref()
                .map(|url| vec![url.clone()])
                .unwrap_or_default();

            let info_urls = vec![data.url.clone()];

            // let video_urls = data
            //     .trailer
            //     .embed_url
            //     .as_ref()
            //     .map(|url| vec![url.clone()])
            //     .unwrap_or_default();

            let image_urls = data
                .images
                .values()
                .map(|image| image.image_url.clone())
                .collect();

            Anime {
                names,
                season: 0,
                episodes: data.episodes.unwrap_or_default(),
                score: data.score.unwrap_or_default(),
                status,
                types,
                platforms,
                genres,
                tags,
                trailer_urls,
                info_urls,
                video_urls: vec![],
                image_urls,
            }
        })
        .collect()
}
