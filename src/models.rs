use serde::{Deserialize, Serialize};

// Struct representing the client class returned by the ENDA backend
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ClientClasses {
    pub id: String,
    pub name: String,

    #[serde(rename = "minScore")]
    pub min_score: Option<i32>,

    #[serde(rename = "maxScore")]
    pub max_score: i32,
}

// Struct representing rewards available in the ENDA loyalty program
#[derive(Debug, Deserialize, Serialize)]
pub struct Reward {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: String,

    #[serde(rename = "type")]
    pub reward_type: String,

    pub status: String,

    #[serde(rename = "manuallyDisabled")]
    pub manually_disabled: bool,

    #[serde(rename = "clientClass")]
    pub client_class: Option<String>,

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
