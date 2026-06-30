use sqlx::{Pool, Postgres};

pub struct EndaServer {
    pub pool: Pool<Postgres>,
}

impl EndaServer {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {pool}
    }
}

pub async fn start(pool: Pool<Postgres>) {
    let _server = EndaServer::new(pool);
}