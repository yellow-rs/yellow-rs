use serenity::prelude::TypeMapKey;
use tokio_postgres::{Client, error::Error, types::FromSql};

use crate::core::game::GameResult;

pub struct DatabaseWrapper {
    client: Client
}

#[derive(FromSql)]
struct Ranking {
    rank: i32,
    ranking_time: chrono::naive::NaiveDateTime,
}

impl DatabaseWrapper {
    pub fn new(client: Client) -> Self {
        DatabaseWrapper {
            client
        }
    }

    pub async fn generate_tables(&self) -> Result<(), Error> {
        // Create type if it doesn't exist
        self.client.execute(r#"DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'ranking') THEN
       CREATE TYPE ranking AS (
            rank integer,
            ranking_time timestamp
        );
    END IF;
END$$;"#, &[]).await?;

    // Create leaderboard table if it doesn't exit
    // the leaderboard table stores the user's score at every timeframe so we can graph it
    self.client.execute(r#"CREATE TABLE IF NOT EXISTS "leaderboard"(
    id bigint PRIMARY KEY NOT NULL,
    rankings ranking[]
);"#, &[]).await?;
Ok(())
    }

    async fn get_rank(&self, id: i64) -> Ranking {
        // If the user doesn't have a score, insert 800 as starting point
        self.client.execute(r#"INSERT INTO leaderboard ("id", "rankings") VALUES ($1::BIGINT, '{"(800, NOW)"}') ON CONFLICT DO NOTHING;"#, &[&id]).await.expect("Failed to insert value into leaderboard");

        // Get score
        self.client.query_one("SELECT leaderboard.rankings FROM leaderboard WHERE id = $1::BIGINT", &[&id]).await.expect("Failed to get ranking from leaderboard").get("rankings")
    }

    pub async fn update_score(&self, a_id: i64, b_id: i64, result: GameResult) {
        let a_rank = self.get_rank(a_id).await.rank;
        let b_rank = self.get_rank(b_id).await.rank;

        //
        //                                     1
        // P(<Player> wins) = ------------------------------------
        //                    1 + 10 ^ ((<B rank> - <A rank>)/400)

        // P(A wins)
        let p_a_wins = 1.0 / (1.0 + 10.0f32.powf((b_rank as f32 + a_rank as f32)/400.0));

        // P(B wins)
        let p_b_wins = 1.0 / (1.0 + 10.0f32.powf((b_rank as f32 + a_rank as f32)/400.0));

        // Real result
        let (a_result, b_result) = result.get_rep();

        // New <Player> ranking = <Player ranking> + 32(<Outcome> - <Expected Outcome>)

        // New A rating
        let new_a_ranking = a_rank + (32.0 * (a_result - p_a_wins)) as i32;
        // New B rating
        let new_b_ranking = b_rank + (32.0 * (b_result - p_b_wins)) as i32;

        println!("a rank: {}, b rank: {}", new_a_ranking, new_b_ranking);
        // TODO: insert the new values
    }
}

impl TypeMapKey for DatabaseWrapper {
    type Value = DatabaseWrapper;
}

