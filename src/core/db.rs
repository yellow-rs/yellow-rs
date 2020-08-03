use serenity::prelude::TypeMapKey;
use tokio_postgres::{Client, error::Error};

use crate::core::game::GameResult;

pub struct DatabaseWrapper {
    client: Client
}

impl DatabaseWrapper {
    pub fn new(client: Client) -> Self {
        DatabaseWrapper {
            client
        }
    }

    pub async fn generate_tables(&self) -> Result<(), Error> {
        self.client.execute(r#"CREATE TABLE IF NOT EXISTS "leaderboard"(
    id serial PRIMARY KEY, ranking INTEGER NOT NULL
);"#, &[]).await?;
        Ok(())
    }

    pub async fn insert_score(&self, id: u64, result: GameResult) {
        
    }
}

impl TypeMapKey for DatabaseWrapper {
    type Value = DatabaseWrapper;
}
