use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct ReadCACertificateRoleResponse {
    pub certificate: String,
    pub display_name: String,
    pub policies: String,
    pub allowed_names: String,
    pub required_extensions: String,
    pub ttl: u32,
    pub max_ttl: u32,
    pub period: u32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ReadCRLResponse {
    pub serials: HashMap<String, ()>,
}
