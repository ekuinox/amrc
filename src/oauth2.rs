use anyhow::{bail, ensure, Result};
use oauth2::{
    basic::{BasicClient, BasicTokenType},
    reqwest::async_http_client,
    url::Url,
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, StandardTokenResponse, TokenUrl,
};
use std::{collections::HashMap, ops::Deref};

#[derive(Debug)]
pub struct OAuth2Authorizer<'a> {
    authorize_url: Url,
    csrf_state: CsrfToken,
    pkce_verifier: PkceCodeVerifier,
    client: &'a OAuth2Client,
}

impl<'a> OAuth2Authorizer<'a> {
    pub fn new(client: &'a OAuth2Client, scopes: Vec<Scope>) -> OAuth2Authorizer {
        let (code_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .set_pkce_challenge(code_challenge)
            .url();
        OAuth2Authorizer {
            authorize_url,
            csrf_state,
            pkce_verifier,
            client,
        }
    }

    pub async fn try_into_token_with_redirect_url(
        self,
        redirect_url: Url,
    ) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
        let params = redirect_url.query_pairs().collect::<HashMap<_, _>>();
        let code = match params.get("code") {
            Some(code) => AuthorizationCode::new(code.to_string()),
            None => bail!("couldn't find pair which key is 'code'"),
        };
        let state = match params.get("state") {
            Some(state) => CsrfToken::new(state.to_string()),
            None => bail!("couldn't find pair which key is 'state'"),
        };
        ensure!(state.secret() == self.csrf_state.secret());
        let token = self
            .client
            .exchange_code(code)
            .set_pkce_verifier(self.pkce_verifier)
            .request_async(async_http_client)
            .await?;
        Ok(token)
    }

    pub fn authorize_url(&self) -> &str {
        self.authorize_url.as_str()
    }
}

#[derive(Debug)]
pub struct OAuth2Client {
    inner: BasicClient,
}

impl Deref for OAuth2Client {
    type Target = BasicClient;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl OAuth2Client {
    pub fn new(
        client_id: String,
        client_secret: String,
        auth_url: AuthUrl,
        token_url: TokenUrl,
        redirect_url: RedirectUrl,
    ) -> OAuth2Client {
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);
        let client = BasicClient::new(client_id, client_secret.into(), auth_url, token_url.into())
            .set_redirect_uri(redirect_url);
        OAuth2Client { inner: client }
    }

    pub fn authorizer(&self, scopes: Vec<Scope>) -> OAuth2Authorizer {
        OAuth2Authorizer::new(self, scopes)
    }
}
