use async_trait::async_trait;
use serenity::all::GuildId;
use sqlx::any::AnyQueryResult;
use sqlx::{Database, Pool};

#[async_trait]
pub trait GuildManager<T, Db: Database> {
    async fn get(pool: &Pool<Db>, id: GuildId) -> sqlx::Result<T>;
    async fn delete(pool: &Pool<Db>, id: GuildId) -> sqlx::Result<AnyQueryResult>;
}
