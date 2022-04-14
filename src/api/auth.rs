use rspotify::{clients::OAuthClient, scopes, AuthCodeSpotify, Config, Credentials, OAuth};
use std::error::Error;

const API_AUTH_REDIRECT_URI: &str = "http://localhost:8888/callback";

pub async fn create_client(
    client_id: &str,
    client_secret: &str,
) -> Result<AuthCodeSpotify, Box<dyn Error>> {
    let creds = Credentials::new(client_id, client_secret);
    let oauth = OAuth {
        redirect_uri: String::from(API_AUTH_REDIRECT_URI),
        scopes: scopes!(
            "user-read-playback-state",
            "user-modify-playback-state",
            "user-read-currently-playing"
        ),
        ..Default::default()
    };
    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, get_client_config()?);

    let url = spotify.get_authorize_url(false)?;
    spotify.prompt_for_token(&url).await?;

    Ok(spotify)
}

fn get_client_config() -> Result<Config, Box<dyn Error>> {
    let dir = xdg::BaseDirectories::new()?;
    let cache_path = dir.place_cache_file("mprispot.json")?;

    Ok(Config {
        cache_path,
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    })
}
