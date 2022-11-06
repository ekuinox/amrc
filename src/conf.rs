use oauth2::{AuthUrl, Scope, TokenUrl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Preset {
    pub client_id: String,
    pub client_secret: String,
    pub base_url: String,
    pub redirect_url: oauth2::RedirectUrl,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub scopes: Vec<Scope>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Conf {
    pub preset: Preset,
    pub access_token: String,
    pub refresh_token: Option<String>,
}
