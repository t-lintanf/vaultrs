use async_trait::async_trait;
use vaultrs::{api::AuthInfo, client::Client, error::ClientError};

use crate::LoginMethod;

struct CertAuthLogin<'a> {
    role: &'a str,
}

#[async_trait]
impl<'a> LoginMethod for CertAuthLogin<'a> {
    async fn login(&self, client: &impl Client, mount: &str) -> Result<AuthInfo, ClientError> {
        vaultrs::auth::tls::login(client, mount, self.role).await
    }
}
