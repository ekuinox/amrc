mod app;
mod oauth2;
mod preset;

use crate::app::App;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::try_parse()?;
    app.run().await
}
