use crate::{
    conf::{Conf, Preset},
    oauth2::OAuth2Client,
};
use anyhow::Result;
use clap::Parser;
use oauth2::{url::Url, TokenResponse};
use std::{
    fs::{File, OpenOptions},
    io::BufWriter,
    path::PathBuf,
};

#[derive(Parser, Debug)]
pub struct AuthSubcommand {
    #[clap(name = "PRESET_FILE_PATH")]
    path: PathBuf,

    #[clap(name = "OUTPUT")]
    output: Option<PathBuf>,
}

impl AuthSubcommand {
    pub async fn run(self) -> Result<()> {
        let reader = File::open(&self.path)?;
        let preset: Preset = serde_yaml::from_reader(reader)?;
        let client = {
            let preset = preset.clone();
            OAuth2Client::new(
                preset.client_id,
                preset.client_secret,
                preset.auth_url,
                preset.token_url,
                preset.redirect_url,
            )
        };
        let authorizer = client.authorizer(preset.scopes.clone());
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

        let access_token = token.access_token().secret().to_owned();
        let refresh_token = token.refresh_token().map(|t| t.secret().to_owned());

        let Some(output) = self.output else {
            println!("access_token: {access_token}");
            if let Some(refresh_token) = refresh_token {
                println!("refresh_token: {refresh_token}");
            }
            return Ok(());
        };
        let writer = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&output)
            .map(BufWriter::new)?;
        let conf = Conf {
            access_token,
            refresh_token,
            preset,
        };

        serde_yaml::to_writer(writer, &conf)?;

        Ok(())
    }
}
