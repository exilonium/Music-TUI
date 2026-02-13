use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use urlencoding::encode;

#[derive(Debug, Clone)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: u64,
}

#[derive(Debug, Clone)]
pub struct SongWithUrl {
    pub song: Song,
    pub stream_url: String,
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
    duration: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ArtistMap {
    primary_artists: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
struct Artist {
    name: String,
}

#[derive(Debug, Deserialize)]
struct SongDetailsResponse {
    songs: Option<Vec<SongDetails>>,
}

#[derive(Debug, Deserialize)]
struct SongDetails {
    more_info: Option<SongDetailsMoreInfo>,
}

#[derive(Debug, Deserialize)]
struct SongDetailsMoreInfo {
    encrypted_media_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct AuthTokenResponse {
    auth_url: Option<String>,
}

pub async fn search(query: &str) -> Result<Vec<Song>> {
    let encoded_query = query.replace(" ", "+");
    let url = format!(
        "https://www.jiosaavn.com/api.php?p=1&q={}&_format=json&_marker=0&api_version=4&ctx=web6dot0&n=20&__call=search.getResults",
        encoded_query
    );
    let client = Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0",
        )
        .build()?;
    let response = client.get(&url).send().await?.json::<ApiResponse>().await?;

    let results = response.results;

    let songs = results
        .into_iter()
        .map(|s| {
            let artist = s
                .more_info
                .as_ref()
                .and_then(|mi| mi.artist_map.as_ref())
                .and_then(|am| am.primary_artists.first())
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "Unknown Artist".to_string());

            let album = s
                .more_info
                .as_ref()
                .and_then(|mi| mi.album.clone())
                .unwrap_or_else(|| "Unknown Album".to_string());

            let duration = s
                .more_info
                .as_ref()
                .and_then(|mi| mi.duration.as_ref())
                .and_then(|d| d.parse::<u64>().ok())
                .unwrap_or(0);

            Song {
                id: s.id,
                title: s.title,
                artist,
                album,
                duration,
            }
        })
        .collect();

    Ok(songs)
}

async fn get_song_details(song_id: &str) -> Result<String> {
    let url = format!(
        "https://www.jiosaavn.com/api.php?__call=song.getDetails&pids={}&api_version=4&_format=json&_marker=0&ctx=web6dot0",
        song_id
    );

    let client = Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0",
        )
        .build()?;
    let response = client
        .get(&url)
        .send()
        .await?
        .json::<SongDetailsResponse>()
        .await?;

    let encrypted_url = response
        .songs
        .and_then(|songs| songs.into_iter().next())
        .and_then(|song| song.more_info)
        .and_then(|info| info.encrypted_media_url)
        .ok_or_else(|| anyhow::anyhow!("Failed to get encrypted media URL"))?;

    Ok(encrypted_url)
}

pub async fn get_stream_url(encrypted_url: &str, bitrate: u32) -> Result<String> {
    let file_suffix = match bitrate {
        12 => "12",
        48 => "48",
        96 => "96",
        160 => "160",
        320 => "320",
        _ => "320",
    };

    // URL encode the encrypted URL (basic implementation)
    let encoded_url = encode(encrypted_url);

    let url = format!(
        "https://www.jiosaavn.com/api.php?__call=song.generateAuthToken&url={}&bitrate={}&api_version=4&_format=json&ctx=web6dot0&_marker=0",
        encoded_url, bitrate
    );

    let client = Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/121.0",
        )
        .build()?;

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<AuthTokenResponse>()
        .await?;

    let auth_url = response
        .auth_url
        .ok_or_else(|| anyhow::anyhow!("Failed to get auth URL"))?;

    // Convert the URL format (web.saavncdn.com -> aac.saavncdn.com)
    // Remove query parameters and change quality suffix
    let converted_url = auth_url
        .split('?')
        .next()
        .unwrap_or(&auth_url)
        .replace("web.saavncdn.com", "aac.saavncdn.com")
        .replace("ac.cf.saavncdn.com", "aac.saavncdn.com")
        .replace("aac.cf.saavncdn.com", "aac.saavncdn.com");

    // Replace the quality suffix in the URL
    let final_url = regex::Regex::new(r"_\d+\.mp4")?
        .replace(&converted_url, &format!("_{}.mp4", file_suffix))
        .to_string();

    Ok(final_url)
}

pub async fn get_song_with_url(song: &Song, bitrate: u32) -> Result<SongWithUrl> {
    let encrypted_url = get_song_details(&song.id).await?;
    let stream_url = get_stream_url(&encrypted_url, bitrate).await?;

    Ok(SongWithUrl {
        song: song.clone(),
        stream_url,
    })
}

pub async fn search_and_get_url(query: &str, bitrate: u32) -> Result<SongWithUrl> {
    let songs = search(query).await?;
    let song = songs
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No songs found"))?;

    get_song_with_url(&song, bitrate).await
}
