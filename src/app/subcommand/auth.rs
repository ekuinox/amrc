use crate::{oauth2::OAuth2Client, preset::Preset};
use anyhow::Result;
use clap::Parser;
use oauth2::{url::Url, TokenResponse};
use std::{fs::File, path::PathBuf};

#[derive(Parser, Debug)]
pub struct AuthSubcommand {
    #[clap(name = "PRESET_FILE_PATH")]
    path: PathBuf,
}

impl AuthSubcommand {
    pub async fn run(self) -> Result<()> {
        let reader = File::open(&self.path)?;
        let preset: Preset = serde_yaml::from_reader(reader)?;
        let client = OAuth2Client::new(
            preset.client_id,
            preset.client_secret,
            preset.auth_url,
            preset.token_url,
            preset.redirect_url,
        );
        let authorizer = client.authorizer(preset.scopes);
        let authorize_url = authorizer.authorize_url();

        println!("authorize_url => {authorize_url}");

        println!("ENTER REDIRECTD URL:");
        let line = {
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line)?;
            line
        };
        let redirected_url = Url::parse(&line)?;

        let token = authorizer
            .try_into_token_with_redirect_url(redirected_url)
            .await?;
        let access_token = token.access_token().secret();
        let refresh_token = token.refresh_token().map(|t| t.secret());

        println!("access_token: {access_token}");
        println!(
            "refresh_token: {}",
            match refresh_token {
                Some(t) => t,
                None => "None",
            }
        );

        Ok(())
    }
}
