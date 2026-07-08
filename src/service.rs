use crate::models::{ClientClasses, Permission, Region, Reward};
use crate::api_client::ApiClient;


// Retrieves client classes from the ENDA backend API
pub async fn get_client_classes(client: &ApiClient,) -> Result<Vec<ClientClasses>, reqwest::Error> {
    
    client.get_json::<Vec<ClientClasses>>("/client-classes").await
   
}

// Retrieves rewards from the ENDA backend API
pub async fn get_rewards(client: &ApiClient,) -> Result<Vec<Reward>, reqwest::Error> {
    
    client.get_json::<Vec<Reward>>("/rewards").await
 
}

// Retrieves regions from the ENDA backend API
pub async fn get_regions(client: &ApiClient,) -> Result<Vec<Region>, reqwest::Error> {
    
    client.get_json::<Vec<Region>>("/regions").await
}

// Retrieves all permissions from the ENDA backend API
pub async fn get_permissions(client: &ApiClient,) -> Result<Vec<Permission>, reqwest::Error> {
    
    client.get_json::<Vec<Permission>>("/permissions").await
}
