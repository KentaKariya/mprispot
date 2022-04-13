use std::error::Error;

mod dbus;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dbus::init().await?;
    loop {}
}
