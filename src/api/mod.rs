use std::error::Error;

use rspotify::{AuthCodeSpotify, ClientResult, prelude::OAuthClient};

mod auth;

pub async fn get_client(
    client_id: &str,
    client_secret: &str,
) -> Result<AuthCodeSpotify, Box<dyn Error>> {
    auth::create_client(client_id, client_secret).await
}

pub async fn pause(client: &AuthCodeSpotify) -> ClientResult<()> {
    client.pause_playback(None).await
}
