use rmcp::ServiceExt;
use rmcp::{
    tool,
    tool_router,
    transport::io::stdio,
};

use sqlx::{Pool, Postgres};

pub struct EndaServer {
    pub pool: Pool<Postgres>,
}
#[tool_router(server_handler)]
impl EndaServer {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {pool}
    }

#[tool(description = "Returns the client classes")]
async fn enda_list_client_classes(&self) -> String{
    
    match crate::database::get_client_classes(&self.pool).await{

        Ok(classes) => {
            let mut output = String::from("Client Classes\n\n");

            for class in classes {

                output.push_str(
                    &format! (
                        "{} ({} - {})\n",
                        class.name,
                        class.min_score.unwrap_or(0),
                        class.max_score
                    )
                );
            }
            output
        }
        Err(error) => {
            format!("Database Error: {}", error)
        }
    }
}
}

pub async fn start(pool: Pool<Postgres>) {

    let server = EndaServer::new(pool);

    // for debugging
    //eprintln!("testing MCP server");

    let service = server
    .serve(stdio())
    .await
    .unwrap();

    service
    .waiting()
    .await
    .unwrap();



}
