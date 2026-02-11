use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: Vec<ApiSong>,
}

#[derive(Debug, Deserialize)]
struct ApiSong {
    id: String,
    title: String,
    more_info: Option<MoreInfo>,
}

#[derive(Debug, Deserialize)]
struct MoreInfo {
    album: Option<String>,

    #[serde(rename = "artistMap")]
    artist_map: Option<ArtistMap>,
}

#[derive(Debug, Deserialize)]
struct ArtistMap {
    primary_artists: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
struct Artist {
    name: String,
}

pub async fn search(query: &str) -> Result<Vec<Song>> {
    let query = query.replace(" ", "+");
    let url = format!(
        "https://www.jiosaavn.com/api.php?p=1&q={}&_format=json&_marker=0&api_version=4&ctx=web6dot0&n=20&__call=search.getResults",
        query
    );
    let response = reqwest::get(url).await?.json::<ApiResponse>().await?;

    let songs = response
        .results
        .into_iter()
        .map(|s| {
            let artist = s
                .more_info
                .as_ref()
                .and_then(|mi| mi.artist_map.as_ref())
                .and_then(|am| am.primary_artists.get(0))
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown Artist".into());

            let album = s
                .more_info
                .as_ref()
                .and_then(|mi| mi.album.clone())
                .unwrap_or_else(|| "Unknown Album".into());

            Song {
                id: s.id,
                title: s.title,
                artist,
                album,
            }
        })
        .collect();
    Ok(songs)
}
