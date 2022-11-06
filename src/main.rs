mod app;

use crate::app::App;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::try_parse()?;
    dbg!(&app);
    app.run().await
}
