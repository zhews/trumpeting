use async_trait::async_trait;
use deadpool_postgres::Pool;
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

struct PostgresKeyDistributionRepository {
    connection_pool: Pool,
}

#[async_trait]
impl KeyDistributionRepository for PostgresKeyDistributionRepository {
    async fn store_public_key(
        &self,
        username: &str,
        public_key: Vec<u8>,
        public_key_signature: Vec<u8>,
    ) -> Result<(), Box<dyn Error>> {
        let client = self.connection_pool.get().await?;
        let statement = client
            .prepare_cached(
                "INSERT INTO public_key (username, public_key, public_key_signature) VALUES ($1, $2, $3)"
            )
            .await?;
        client
            .execute(&statement, &[&username, &public_key, &public_key_signature])
            .await?;
        Ok(())
    }
    async fn get_public_key(&self, username: &str) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
        let client = self.connection_pool.get().await?;
        let statement = client
            .prepare_cached(
                "SELECT public_key, public_key_signature FROM public_key WHERE username = $1",
            )
            .await?;
        let result = client.query_one(&statement, &[&username]).await?;
        let public_key = result.try_get("public_key")?;
        let public_key_signature = result.try_get("public_key_signature")?;
        Ok((public_key, public_key_signature))
    }
}
