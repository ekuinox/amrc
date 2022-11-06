use oauth2::{AuthUrl, Scope, TokenUrl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Preset {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: oauth2::RedirectUrl,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub scopes: Vec<Scope>,
}
