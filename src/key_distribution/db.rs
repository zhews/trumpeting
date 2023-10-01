use async_trait::async_trait;
use std::error::Error;

#[async_trait]
trait KeyDistributionRepository {
    async fn store_public_key(
        &self,
        username: &str,
        public_key: Vec<u8>,
        public_key_signature: Vec<u8>,
    ) -> Result<(), Box<dyn Error>>;
    async fn get_public_key(&self, username: &str) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>>;
}
