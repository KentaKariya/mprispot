use rspotify::model::{FullEpisode, FullTrack, PlayableItem};
use std::collections::HashMap;
use zbus::zvariant::Value;

use super::{SpotifyError, SpotifyResult};

pub struct Metadata {
    pub track_id: String,
}

impl From<Metadata> for HashMap<String, Value<'_>> {
    fn from(m: Metadata) -> Self {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert(String::from("mpris:trackid"), m.track_id.into());
        map
    }
}

impl TryFrom<PlayableItem> for Metadata {
    type Error = SpotifyError;

    fn try_from(i: PlayableItem) -> Result<Self, Self::Error> {
        match i {
            PlayableItem::Track(t) => Metadata::try_from(t),
            PlayableItem::Episode(e) => Metadata::try_from(e),
        }
    }
}

impl TryFrom<FullTrack> for Metadata {
    type Error = SpotifyError;

    fn try_from(t: FullTrack) -> SpotifyResult<Metadata> {
        match t.id {
            Some(i) => Ok(Metadata {
                track_id: i.to_string(),
            }),
            None => Err(SpotifyError::Parse(String::from(
                "Could not read metadata of track",
            ))),
        }
    }
}

impl TryFrom<FullEpisode> for Metadata {
    type Error = SpotifyError;

    fn try_from(e: FullEpisode) -> Result<Self, Self::Error> {
        Ok(Metadata {
            track_id: e.id.to_string(),
        })
    }
}
