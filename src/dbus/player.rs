use std::{collections::HashMap, sync::Arc};

use rspotify::AuthCodeSpotify;
use zbus::{dbus_interface, zvariant::Value};

use crate::api;
use crate::api::Metadata;

// Cannot use derive macros for properties
// https://dbus.pages.freedesktop.org/zbus/faq.html#why-do-async-tokio-api-calls-from-interface-methods-not-work
impl From<Metadata> for HashMap<String, Value<'_>> {
    fn from(m: Metadata) -> Self {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert(String::from("mpris:trackid"), m.track_id.into());
        map.insert(String::from("xesam:title"), m.title.into());
        map.insert(String::from("xesam:album"), m.album.into());
        map
    }
}

pub struct PlayerIface {
    client: Arc<AuthCodeSpotify>,
}

impl PlayerIface {
    pub fn new(client: Arc<AuthCodeSpotify>) -> Self {
        Self { client }
    }
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.Player")]
impl PlayerIface {
    async fn play(&self) {
        let _ = api::play(&self.client).await;
    }

    async fn pause(&self) {
        let _ = api::pause(&self.client).await;
    }

    async fn play_pause(&self) {
        let _ = api::play_pause(&self.client).await;
    }

    async fn seek(&self, offset: i64) {
        let _ = api::seek(&self.client, offset).await;
    }

    #[dbus_interface(property)]
    async fn metadata(&self) -> HashMap<String, Value<'_>> {
        match api::metadata(&self.client).await {
            Ok(m) => m.into(),
            Err(_) => HashMap::new(),
        }
    }

    #[dbus_interface(property)]
    fn can_play(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_pause(&self) -> bool {
        true
    }

    #[dbus_interface(property)]
    fn can_seek(&self) -> bool {
        true
    }
}
