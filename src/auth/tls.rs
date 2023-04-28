use crate::api;
use crate::api::auth::tls::requests::{
    ConfigureTLSCertificateMethodRequest, ConfigureTLSCertificateMethodRequestBuilder,
    LoginWithTLSCertificateRequest,
};
use crate::api::AuthInfo;
use crate::client::Client;
use crate::error::ClientError;

/// Configure TLS authentication
///
/// See [ConfigureTLSCertificateMethodRequest]
#[instrument(skip(client, opts), err)]
pub async fn configure(
    client: &impl Client,
    mount: &str,
    opts: Option<&mut ConfigureTLSCertificateMethodRequestBuilder>,
) -> Result<(), ClientError> {
    let mut t = ConfigureTLSCertificateMethodRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).mount(mount).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Login with TLS Certificate Method
///
/// See [LoginWithTLSCertificateRequest]
#[instrument(skip(client), err)]
pub async fn login(client: &impl Client, mount: &str, name: &str) -> Result<AuthInfo, ClientError> {
    let endpoint = LoginWithTLSCertificateRequest::builder()
        .mount(mount)
        .name(name)
        .build()
        .unwrap();
    api::auth(client, endpoint).await
}

pub mod certs {

    use crate::api;
    use crate::api::auth::tls::requests::{
        CreateCACertificateRoleRequest, CreateCACertificateRoleRequestBuilder,
        DeleteCertificateRoleRequest, ListCertificateRolesRequest, ReadCertificateRoleRequest,
    };
    use crate::api::auth::tls::responses::ReadCACertificateRoleResponse;
    use crate::client::Client;
    use crate::error::ClientError;

    /// Create CA Certificate Role
    ///
    /// See [CreateCACertificateRoleRequest]
    #[instrument(skip(client, opts), err)]
    pub async fn create(
        client: &impl Client,
        mount: &str,
        role: &str,
        opts: Option<&mut CreateCACertificateRoleRequestBuilder>,
    ) -> Result<(), ClientError> {
        let mut t = CreateCACertificateRoleRequest::builder();
        let endpoint = opts
            .unwrap_or(&mut t)
            .mount(mount)
            .role(role)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// List Certificate Roles
    ///
    /// See [ListCertificateRolesRequest]
    #[instrument(skip(client), err)]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<Option<Vec<String>>, ClientError> {
        let endpoint = ListCertificateRolesRequest::builder()
            .mount(mount)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Read CA Certificate Role
    ///
    /// See [ReadCertificateRoleRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        role: &str,
    ) -> Result<Option<ReadCACertificateRoleResponse>, ClientError> {
        let endpoint = ReadCertificateRoleRequest::builder()
            .mount(mount)
            .role(role)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Delete Certificate Role
    ///
    /// See [DeleteCertificateRoleRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(client: &impl Client, mount: &str, role: &str) -> Result<(), ClientError> {
        let endpoint = DeleteCertificateRoleRequest::builder()
            .mount(mount)
            .role(role)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }
}
pub mod crls {
    use crate::api;
    use crate::api::auth::tls::requests::{
        CreateCRLRequest, DeleteCRLRequest, ListCRLsRequest, ReadCRLRequest,
    };
    use crate::api::auth::tls::responses::ReadCRLResponse;
    use crate::client::Client;
    use crate::error::ClientError;

    /// Create CRL
    ///
    /// See [CreateCRLRequest]
    #[instrument(skip(client), err)]
    pub async fn create(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = CreateCRLRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// List CRLs
    ///
    /// See [ListCRLsRequest]
    #[instrument(skip(client), err)]
    pub async fn list(
        client: &impl Client,
        mount: &str,
    ) -> Result<Option<Vec<String>>, ClientError> {
        let endpoint = ListCRLsRequest::builder().mount(mount).build().unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Read CRL
    ///
    /// See [ReadCRLRequest]
    #[instrument(skip(client), err)]
    pub async fn read(
        client: &impl Client,
        mount: &str,
        name: &str,
    ) -> Result<Option<ReadCRLResponse>, ClientError> {
        let endpoint = ReadCRLRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }

    /// Delete CRL
    ///
    /// See [DeleteCRLRequest]
    #[instrument(skip(client), err)]
    pub async fn delete(client: &impl Client, mount: &str, name: &str) -> Result<(), ClientError> {
        let endpoint = DeleteCRLRequest::builder()
            .mount(mount)
            .name(name)
            .build()
            .unwrap();
        api::exec_with_result(client, endpoint).await
    }
}
