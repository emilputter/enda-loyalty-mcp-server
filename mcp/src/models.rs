use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Struct representing the client class returned by the ENDA backend
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ClientClasses {
    pub id: String,
    pub name: String,

    #[serde(rename = "minScore")]
    pub min_score: Option<i32>,

    #[serde(rename = "maxScore")]
    pub max_score: Option<i32>,
}

// Struct representing rewards available in the ENDA loyalty program
#[derive(Debug, Deserialize, Serialize)]
pub struct Reward {
    pub id: String,
    pub code: String,
    pub name: String,

    pub description: Option<String>,

    #[serde(rename = "type")]
    pub reward_type: String,

    pub status: String,

    #[serde(rename = "manuallyDisabled")]
    pub manually_disabled: bool,

    #[serde(rename = "clientClass")]
    pub client_class: Option<ClientClasses>,

    pub category: String,

    #[serde(rename = "pointsCost")]
    pub points_cost: i32,

    pub stock: i32,

    #[serde(rename = "validFrom")]
    pub valid_from: String,

    #[serde(rename = "validUntil")]
    pub valid_until: String,

    pub image: String,

    #[serde(rename = "createdAt")]
    pub created_at: String,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,

    #[serde(rename = "pendingRequestsCount")]
    pub pending_requests_count: Option<i32>,
}

// Struct representing the region returned by the ENDA backend
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
}



#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentUser {
    pub sub: String,
    pub email: String,
    pub name: String,

    #[serde(rename = "preferred_username")]
    pub preferred_username: String,

    #[serde(rename = "given_name")]
    pub given_name: String,

    #[serde(rename = "family_name")]
    pub family_name: String,

    #[serde(rename = "local_user_id")]
    pub local_user_id: String,

    #[serde(rename = "local_user_created")]
    pub local_user_created: bool,

    #[serde(rename = "points_available")]
    pub points_available: i32,

    #[serde(rename = "points_blocked")]
    pub points_blocked: i32,

    #[serde(rename = "assignedRole")]
    pub assigned_role: AssignedRole,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssignedRole {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,

    #[serde(rename = "isActive")]
    pub is_active: bool,

    pub permissions: Vec<Permission>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Permission {
    pub code: String,
    pub module: String,
    pub action: String,
    pub resource: String,
    pub scope: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateRoleRequest {
    pub code: String,

    pub description: String,

    #[serde(rename = "isActive")]
    pub is_active: bool,

    pub name: String,

    #[serde(rename = "permissionCodes")]
    pub permission_codes: Vec<String>,
}