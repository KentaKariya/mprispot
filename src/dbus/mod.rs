use zbus::{Connection, ConnectionBuilder, Error};

mod base;

const MPRIS_PREFIX: &str = "org.mpris.MediaPlayer2";
const MPRIS_NAME: &str = "mprispot";

pub async fn init() -> Result<Connection, Error> {
    let base_iface = base::BaseIface::new();
    ConnectionBuilder::session()?
        .name(format!("{}.{}", MPRIS_PREFIX, MPRIS_NAME))?
        .serve_at("/org/mpris/MediaPlayer2", base_iface)?
        .build()
        .await
}
