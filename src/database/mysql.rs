use sqlx::MySqlPool;

pub async fn mysql(url: String) -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(&url).await
}