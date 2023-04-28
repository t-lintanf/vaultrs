use super::responses::*;
use rustify_derive::Endpoint;
use serde::Serialize;

/// ## Create CA Certificate Role
/// Sets a CA cert and associated parameters in a role name.
///
/// * Method POST
/// * Path {self.mount}/certs/{self.role}
/// * Response: N/A
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#create-ca-certificate-role
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/certs/{self.role}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateCACertificateRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role: String,
    pub certificate: String,
    pub allowed_names: Option<String>,
    pub allowed_common_names: Option<Vec<String>>,
    pub allowed_dns_sans: Option<Vec<String>>,
    pub allowed_email_sans: Option<Vec<String>>,
    pub allowed_uri_sans: Option<Vec<String>>,
    pub allowed_organizational_units: Option<Vec<String>>,
    pub required_extensions: Option<Vec<String>>,
    pub allowed_metadata_extensions: Option<Vec<String>>,
    pub ocsp_enabled: Option<bool>,
    pub ocsp_ca_certificates: Option<String>,
    pub ocsp_servers_override: Option<Vec<String>>,
    pub ocsp_fail_open: Option<bool>,
    pub ocsp_query_all_servers: Option<bool>,
    pub display_name: Option<String>,
    pub token_ttl: Option<String>,
    pub token_max_ttl: Option<String>,
    pub token_policies: Option<Vec<String>>,
    pub policies: Option<Vec<String>>,
    pub token_bound_cidrs: Option<Vec<String>>,
    pub token_explicit_max_ttl: Option<String>,
    pub token_no_default_policy: Option<bool>,
    pub token_num_uses: Option<String>,
    pub token_period: Option<String>,
    pub token_type: Option<String>,
}

/// ## Read CA Certificate Role
/// Gets information associated with the named role.
///
/// * Method: GET
/// * Path: /auth/{self.mount}/certs/{self.role}
/// * Response: [Option<ReadCACertificateRoleResponse>]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#read-ca-certificate-role
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/certs/{self.role}",
    method = "GET",
    response = "Option<ReadCACertificateRoleResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadCertificateRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role: String,
}

/// ## List Certificate Roles
/// Lists configured certificate names.
///
/// * Method: GET
/// * Path: /auth/{self.mount}/certs
/// * Response: [Option<Vec<String>>]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#list-certificate-roles
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/certs",
    method = "GET",
    response = "Option<Vec<String>>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListCertificateRolesRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Delete Certificate Role
/// Deletes the named role and CA cert from the method mount.
///
/// * Method: GET
/// * Path: /auth/{self.mount}/certs/{self.name}
/// * Response: N/A
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#delete-certificate-role
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/certs/{self.role}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteCertificateRoleRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub role: String,
}

/// ## List CRLs
/// Lists configured certificate revocation lists.
/// * Method: GET
/// * Path: /auth/{self.mount}/crls
/// * Response: [Option<Vec<String>>]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#list-crls
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/crls",
    method = "GET",
    response = "Option<Vec<String>>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListCRLsRequest {
    #[endpoint(skip)]
    pub mount: String,
}

/// ## Create CRL
/// Sets a named CRL.
/// * Method: POST
/// * Path: /auth/{self.mount}/crls/{self.name}
/// * Response: N/A
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#create-crl
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/crls/{self.name}",
    method = "POST",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct CreateCRLRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
    pub crl: String,
}

/// ## Read CRL
/// Gets information associated with the named CRL (currently, the serial numbers contained within).
/// As the serials can be integers up to an arbitrary size, these are returned as strings.
/// * Method: GET
/// * Path: /auth/{self.mount}/crls/{self.name}
/// * Response: [Option<ReadCRLResponse>]
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#read-crl
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/crls/{self.name}",
    method = "GET",
    response = "Option<ReadCRLResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadCRLRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Delete CRL
/// Deletes the named CRL from the auth method mount.
/// * Method: DELETE
/// * Path: /auth/{self.mount}/crls/{self.name}
/// * Response: N/A
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#delete-crl
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(
    path = "/auth/{self.mount}/crls/{self.name}",
    method = "DELETE",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteCRLRequest {
    #[endpoint(skip)]
    pub mount: String,
    #[endpoint(skip)]
    pub name: String,
}

/// ## Configure TLS Certificate Method
/// Deletes the named CRL from the auth method mount.
/// * Method: GET
/// * Path: /auth/{self.mount}/config
/// * Response: N/A
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#configure-tls-certificate-method
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "/auth/{self.mount}/config", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ConfigureTLSCertificateMethodRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub disable_binding: Option<bool>,
    pub enable_identity_alias_metadata: Option<bool>,
    pub ocsp_cache_size: Option<u32>,
}

/// ## Login with TLS Certificate Method
/// * Method: GET
/// * Path: /auth/{self.mount}/login
/// * Response: N/A
/// * Reference: https://developer.hashicorp.com/vault/api-docs/auth/cert#login-with-tls-certificate-method
#[derive(Builder, Debug, Default, Endpoint, Serialize)]
#[endpoint(path = "/auth/{self.mount}/login", method = "POST", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct LoginWithTLSCertificateRequest {
    #[endpoint(skip)]
    pub mount: String,
    pub name: String,
}
