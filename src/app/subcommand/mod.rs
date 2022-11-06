mod auth;
mod init;
mod request;

use self::{auth::AuthSubcommand, init::InitSubcommand, request::RequestSubcommand};
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum Subcommand {
    #[clap(name = "init")]
    Init(InitSubcommand),
    #[clap(name = "auth")]
    Auth(AuthSubcommand),
    #[clap(name = "req")]
    Req(RequestSubcommand),
}

impl Subcommand {
    pub async fn run(self) -> Result<()> {
        match self {
            Subcommand::Init(s) => s.run().await,
            Subcommand::Auth(s) => s.run().await,
            Subcommand::Req(s) => s.run().await,
        }
    }
}
