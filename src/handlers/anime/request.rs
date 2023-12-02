#![allow(dead_code)]

use crate::constructor::myanimelist::anime::Anime;

async fn module_request_jikan(req: String) -> String {
    let client = reqwest::Client::new();

    let url = format!("{:?}{}", "https://api.jikan.moe/v4/anime?q=", req);

    let body = client.get(url).send().await.unwrap();

    let res: String = format!("{:?}", body.text().await.unwrap());

    // let res: Anime = body.json().await.unwrap();

    return res;
}
