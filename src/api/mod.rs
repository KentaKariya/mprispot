use rspotify::{
    model::{AdditionalType, CurrentlyPlayingContext},
    prelude::OAuthClient,
    AuthCodeSpotify, ClientError,
};
use thiserror::Error;

mod auth;

#[derive(Debug, Error)]
pub enum SpotifyError {
    #[error("Spotify could not handle the request")]
    Api(#[from] ClientError),

    #[error("{0}")]
    Parse(String),
}

type SpotifyResult<T> = Result<T, SpotifyError>;

const ALL_TYPES: [AdditionalType; 2] = [AdditionalType::Track, AdditionalType::Episode];

pub async fn get_client(
    client_id: &str,
    client_secret: &str,
) -> Result<AuthCodeSpotify, Box<dyn std::error::Error>> {
    auth::create_client(client_id, client_secret).await
}

pub async fn play(client: &AuthCodeSpotify) -> SpotifyResult<()> {
    Ok(client.resume_playback(None, None).await?)
}

pub async fn pause(client: &AuthCodeSpotify) -> SpotifyResult<()> {
    Ok(client.pause_playback(None).await?)
}

pub async fn play_pause(client: &AuthCodeSpotify) -> SpotifyResult<()> {
    let context = get_currently_playing(client).await?;
    match context.is_playing {
        true => pause(client).await,
        false => play(client).await,
    }
}

pub async fn seek(client: &AuthCodeSpotify, offset_us: i64) -> SpotifyResult<()> {
    let context = get_currently_playing(client).await?;
    let current_us = context
        .progress
        .map(|t| t.as_micros())
        .and_then(|u| i64::try_from(u).ok())
        .ok_or_else(|| SpotifyError::Parse(String::from(
            "Could not read track progress",
        )))?;

    let target_us = (current_us + offset_us).max(0) as u64;
    skip_to_position(client, target_us).await
}

pub async fn skip_to_position(client: &AuthCodeSpotify, target_us: u64) -> SpotifyResult<()> {
    let target_ms = u32::try_from(target_us / 1_000).unwrap();
    Ok(client.seek_track(target_ms, None).await?)
}

async fn get_currently_playing(client: &AuthCodeSpotify) -> SpotifyResult<CurrentlyPlayingContext> {
    client
        .current_playing(None, Some(&ALL_TYPES))
        .await?
        .ok_or_else(|| SpotifyError::Parse(String::from(
            "Could not get currently playing track",
        )))
}
