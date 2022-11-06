mod subcommand;

use self::subcommand::Subcommand;
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct App {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

impl App {
    pub async fn run(self) -> Result<()> {
        self.subcommand.run().await
    }
}
