use zbus::dbus_interface;

pub struct BaseIface;

impl BaseIface {
    pub fn new() -> Self {
        Self {}
    }
}

#[dbus_interface(name = "org.mpris.MediaPlayer2")]
impl BaseIface {
    fn quit(&self) {
        std::process::exit(0);
    }

    #[dbus_interface(property)]
    fn can_raise(&self) -> bool {
        false
    }

    #[dbus_interface(property)]
    fn can_quit(&self) -> bool {
        true
    }
}
