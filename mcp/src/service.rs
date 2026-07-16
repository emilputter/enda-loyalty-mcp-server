use crate::api_client::{ApiClient, ApiError};
use crate::models::{
    ClientClasses,
    CreateRoleRequest,
    CurrentUser,
    Permission,
    Region,
    Reward,
};

// Retrieves all client classes from the ENDA API
pub async fn get_client_classes(client: &ApiClient) -> Result<Vec<ClientClasses>, ApiError> {
    client
        .get_json::<Vec<ClientClasses>>("/client-classes")
        .await
}

// Retrieves all rewards from the ENDA API
pub async fn get_rewards(client: &ApiClient) -> Result<Vec<Reward>, ApiError> {
    client.get_json::<Vec<Reward>>("/rewards").await
}

// Retrieves all regions from the ENDA API
pub async fn get_regions(client: &ApiClient) -> Result<Vec<Region>, ApiError> {
    client.get_json::<Vec<Region>>("/regions").await
}

// Retrieves the current authenticated user
pub async fn get_current_user(client: &ApiClient) -> Result<CurrentUser, ApiError> {
    client.get_json::<CurrentUser>("/auth/me").await
}

// Retrieves all permissions from the ENDA API
pub async fn get_permissions(
    client: &ApiClient,
) -> Result<Vec<Permission>, ApiError> {
    client
        .get_json::<Vec<Permission>>("/permissions")
        .await
}

// Creates a new role in the ENDA backend
pub async fn create_role(
    client: &ApiClient,
    role: &CreateRoleRequest,
) -> Result<serde_json::Value, ApiError> {

    client
        .post_json("/roles", role)
        .await
}
