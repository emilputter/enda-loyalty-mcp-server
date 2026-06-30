use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::models::ClientClasses;

pub async fn connect() -> Pool<Postgres>{
    eprintln! ("Connection to PostgreSQL");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:<password>@localhost/enda")
    .await
    .expect("Failed to connect to PostgreSQL");

    eprintln!("connected");

    pool
}

pub async fn get_client_classes(
    pool: &Pool<Postgres>,
) -> Result<Vec<ClientClasses>, sqlx::Error>{

    let classes = sqlx::query_as::<_, ClientClasses>(
        "
        SELECT
        id,
        name,
        min_score,
        max_score,
        FROM client_classes
        "
    )
    .fetch_all(pool)
    .await?;

    Ok(classes)


}