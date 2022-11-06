use super::{
    wrapped::{AuthUrl, RedirectUrl, TokenUrl},
    OAuth2Client,
};
use anyhow::Result;
use clap::Parser;
use oauth2::{url::Url, Scope, TokenResponse};

#[derive(Parser, Debug)]
pub struct App {
    #[clap(long = "auth-url")]
    auth_url: AuthUrl,

    #[clap(long = "token-url")]
    token_url: TokenUrl,

    #[clap(long = "redirect-url", default_value = "http://localhost:8080/callback")]
    redirect_url: RedirectUrl,

    #[clap(long = "client-id")]
    client_id: String,

    #[clap(long = "client-secret")]
    client_secret: String,

    #[clap(short = 's', long = "scope")]
    scopes: Vec<String>,
}

impl App {
    pub async fn run(self) -> Result<()> {
        let scopes = self.scopes.into_iter().map(Scope::new).collect::<Vec<_>>();
        let client = OAuth2Client::new(
            self.client_id,
            self.client_secret,
            self.auth_url.into(),
            self.token_url.into(),
            self.redirect_url.into(),
        );
        let authorizer = client.authorizer(scopes);
        let authorize_url = authorizer.authorize_url();

        println!("authorize_url => {authorize_url}");

        let line = {
            let mut line = String::new();
            let _ = std::io::stdin().read_line(&mut line)?;
            line
        };
        let redirected_url = Url::parse(&line)?;

        let token = authorizer
            .try_into_token_with_redirect_url(&client, redirected_url)
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
