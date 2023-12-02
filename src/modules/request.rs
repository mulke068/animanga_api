#![allow(dead_code)]

enum AnimeUri {
    JIKAN,
    MYANIMELIST,
    ANIME9,
    CRUNCHYROLL,
}

enum MangaUri {
    JIKAN,
    MYANIMELIST,
    MANGADEX,
}

#[derive(Debug)]
enum Uri {
    MangaUri,
    AnimeUri,
}

async fn module_request_jikan(uri: Uri, req: String) -> String {
    let client = reqwest::Client::new();

    let url = format!("{:?}{}", uri, req);

    let body = client.get(url).send().await.unwrap();

    let res = format!("{:?}", body.text().await.unwrap());

    return res;
}
