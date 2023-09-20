// pub mod data;

pub mod data {
    pub use spog_model::prelude::*;
}

mod access_token;
mod analyze;
mod config;
mod cve;
mod hooks;
mod pkg;
mod sbom;
mod search;
mod version;
mod vuln;

pub use access_token::*;
pub use analyze::*;
pub use config::*;
pub use cve::*;
pub use hooks::*;
pub use pkg::*;
pub use sbom::*;
pub use search::*;
pub use version::*;
pub use vuln::*;

use url::Url;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Backend {
    pub endpoints: Endpoints,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct OpenIdConnect {
    pub issuer: String,
    #[serde(default = "default::client_id")]
    pub client_id: String,
    #[serde(default = "default::scopes")]
    pub scopes: String,
    #[serde(default = "default::after_logout")]
    pub after_logout: String,
}

impl OpenIdConnect {
    pub fn scopes(&self) -> Vec<String> {
        self.scopes.split(' ').map(|s| s.to_string()).collect()
    }
}

mod default {
    pub fn client_id() -> String {
        "frontend".to_string()
    }

    pub fn scopes() -> String {
        "openid".to_string()
    }

    pub fn after_logout() -> String {
        "/notloggedin".to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Endpoints {
    pub url: Url,
    pub bombastic: Url,
    pub vexination: Url,

    pub oidc: OpenIdConnect,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_write_key: Option<String>,
}

impl Endpoints {
    pub fn get(&self, endpoint: Endpoint) -> &Url {
        match endpoint {
            Endpoint::Api => &self.url,
            Endpoint::Vexination => &self.vexination,
            Endpoint::Bombastic => &self.bombastic,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Endpoint {
    Api,
    Vexination,
    Bombastic,
}

impl Backend {
    pub fn join(&self, endpoint: Endpoint, input: &str) -> Result<Url, url::ParseError> {
        self.endpoints.get(endpoint).join(input)
    }
}