use crate::preset::Preset;
use anyhow::Result;
use clap::Parser;
use oauth2::{AuthUrl, RedirectUrl, Scope, TokenUrl};
use std::{fs::OpenOptions, io::BufWriter, path::PathBuf};

#[derive(Parser, Debug)]
pub struct InitSubcommand {
    #[clap(name = "PRESET_FILE_PATH")]
    path: PathBuf,

    #[clap(short = 'a', long = "auth-url")]
    auth_url: Option<String>,

    #[clap(short = 't', long = "token-url")]
    token_url: Option<String>,

    #[clap(
        short = 'r',
        long = "redirect-url",
        default_value = "http://localhost:8080/callback"
    )]
    redirect_url: Option<String>,

    #[clap(short = 'i', long = "client-id")]
    client_id: Option<String>,

    #[clap(short = 'e', long = "client-secret")]
    client_secret: Option<String>,

    #[clap(short = 's', long = "scope")]
    scopes: Vec<String>,
}

impl InitSubcommand {
    pub async fn run(self) -> Result<()> {
        let client_id = self.client_id.unwrap_or_else(|| "CLIENT_ID".into());
        let client_secret = self.client_secret.unwrap_or_else(|| "CLIENT_SECRET".into());
        let redirect_url = RedirectUrl::new(
            self.redirect_url
                .unwrap_or_else(|| "http://localhost:8080/callback".into()),
        )?;
        let auth_url = AuthUrl::new(self.auth_url.unwrap_or_else(|| "http://AUTH_URL".into()))?;
        let token_url = TokenUrl::new(self.token_url.unwrap_or_else(|| "http://TOKEN_URL".into()))?;
        let scopes = if self.scopes.is_empty() {
            vec![Scope::new("SCOPE".into())]
        } else {
            self.scopes.into_iter().map(Scope::new).collect::<Vec<_>>()
        };

        let preset = Preset {
            client_id,
            client_secret,
            redirect_url,
            auth_url,
            token_url,
            scopes,
        };

        let writer = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.path)
            .map(BufWriter::new)?;
        serde_yaml::to_writer(writer, &preset)?;

        Ok(())
    }
}
