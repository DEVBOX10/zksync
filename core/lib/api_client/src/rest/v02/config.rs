use crate::rest::client::{Client, Result};
use zksync_types::api_v02::Response;

/// Configuration API part.
impl Client {
    pub async fn config_v02(&self) -> Result<Response> {
        self.get_with_scope(super::API_V02_SCOPE, "config")
            .send()
            .await
    }
}
