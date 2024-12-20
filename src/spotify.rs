use crate::gibmon_config::SpotifyConfig;
use base64::{Engine, engine::general_purpose};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(serde::Deserialize, Debug)]
pub struct SpotifyTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

pub async fn fetch_spotify_token(
    config: &SpotifyConfig,
) -> Result<SpotifyTokenResponse, reqwest::Error> {
    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", "AQAOyAIK0jwWJ8fdj7T1NKZWYeWMsRcRVx0OyqWA1PSa76-HJx42QUl8AiU45vrg23GVLpvnkeCdhbojYhafnFhJ39UvihKPc6elmjz6BitmPlBd_-JeMt5wD5taQUqa_eg8QW25g_QFfjR8Gb69bFHA9iFoOup_Y3DfCAxGV6kFLUFAiGEn0zEfXPgSiM1FbIFBqBfj");
    params.insert("redirect_uri", "http://localhost:4202");

    let auth_header = format!(
        "Basic {}",
        general_purpose::STANDARD.encode(format!("{}:{}", config.ClientId, config.ClientSecret))
    );

    let request = client
        .post("https://accounts.spotify.com/api/token")
        .header("Authorization", auth_header)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .build()?;

    // Print the request details
    println!("Request URL: {}", request.url());
    println!("Request Headers: {:?}", request.headers());
    if let Some(body) = request.body() {
        if let Some(bytes) = body.as_bytes() {
            println!("Request Body: {}", String::from_utf8_lossy(bytes));
        } else {
            println!("Request Body: Non-string body (possibly a stream)");
        }
    }

    // Send the request and handle response
    let response = client.execute(request).await?;
    let status = response.status();

    // Check if the response is a success
    if let Err(e) = response.error_for_status_ref() {
        // There's an HTTP error, so let's still read the body for logging
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error body".to_string());
        eprintln!("HTTP Error Status: {}", status);
        eprintln!("Error Body: {}", error_body);
        return Err(e);
    }

    // Deserialize the response
    let token_response: SpotifyTokenResponse = response.json().await?;
    Ok(token_response)
}

#[derive(Deserialize, Debug)]
pub struct SpotifyCurrentlyPlaying {
    pub item: Option<Track>,      // The track currently playing, if any
    pub is_playing: bool,         // Whether something is playing
    pub progress_ms: Option<u64>, // Progress into the currently playing track
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub name: String,                // Track name
    pub duration_ms: u64,            // Track duration
    pub artists: Vec<Artist>,        // List of artists
    pub album: Album,                // Album details
    pub external_urls: ExternalUrls, // External Spotify links
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub name: String,                // Artist name
    pub external_urls: ExternalUrls, // External Spotify links
}

#[derive(Deserialize, Debug)]
pub struct Album {
    pub name: String,                // Album name
    pub images: Vec<Image>,          // Album cover images
    pub external_urls: ExternalUrls, // External Spotify links
}

#[derive(Deserialize, Debug)]
pub struct ExternalUrls {
    pub spotify: String, // Spotify URL
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub url: String,         // Image URL
    pub height: Option<u32>, // Image height
    pub width: Option<u32>,  // Image width
}

pub async fn fetch_currently_playing(
    spotify_token: &SpotifyTokenResponse,
) -> Result<Option<SpotifyCurrentlyPlaying>, reqwest::Error> {
    let client = Client::new();

    let response = client
        .get("https://api.spotify.com/v1/me/player/currently-playing")
        .header(
            "Authorization",
            format!("Bearer {}", spotify_token.access_token),
        )
        .send()
        .await?
        .error_for_status()?;

    // If the response body is empty (nothing playing), return None
    if response.content_length() == Some(0) {
        return Ok(None);
    }

    let currently_playing: SpotifyCurrentlyPlaying = response.json().await?;
    Ok(Some(currently_playing))
}
