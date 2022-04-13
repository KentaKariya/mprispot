use std::sync::Arc;

use rspotify::AuthCodeSpotify;
use zbus::{Connection, ConnectionBuilder, Error};

mod base;
mod player;

const MPRIS_PREFIX: &str = "org.mpris.MediaPlayer2";
const MPRIS_NAME: &str = "mprispot";

pub async fn init(client: Arc<AuthCodeSpotify>) -> Result<Connection, Error> {
    let base_iface = base::BaseIface::new();
    let player_iface = player::PlayerIface::new(client);
    ConnectionBuilder::session()?
        .name(format!("{}.{}", MPRIS_PREFIX, MPRIS_NAME))?
        .serve_at("/org/mpris/MediaPlayer2", base_iface)?
        .serve_at("/org/mpris/MediaPlayer2", player_iface)?
        .build()
        .await
}
