mod auth;
mod init;

use self::{auth::AuthSubcommand, init::InitSubcommand};
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Subcommand {
    #[clap(name = "init")]
    Init(InitSubcommand),
    #[clap(name = "auth")]
    Auth(AuthSubcommand),
}

impl Subcommand {
    pub async fn run(self) -> Result<()> {
        match self {
            Subcommand::Init(s) => s.run().await,
            Subcommand::Auth(s) => s.run().await,
        }
    }
}
