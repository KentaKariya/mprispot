use std::{error::Error, future, sync::Arc};

use clap::Parser;

mod api;
mod dbus;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short = 'i', long)]
    client_id: String,

    #[clap(short = 's', long)]
    client_secret: String,
}

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let client = api::get_client(&args.client_id, &args.client_secret).await?;
    let _ = dbus::init(Arc::new(client)).await?;
    future::pending::<()>().await;

    Ok(())
}
