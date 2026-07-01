use crate::models::{ClientClasses, Region, Reward};

// Retrieves client classes from the ENDA backend API
pub async fn get_client_classes() -> Result<Vec<ClientClasses>, reqwest::Error> {
    let classes = reqwest::get("https://api.hederacourt.site/api/v1/client-classes")
        .await?
        .json::<Vec<ClientClasses>>()
        .await?;

    Ok(classes)
}

// Retrieves rewards from the ENDA backend API
pub async fn get_rewards() -> Result<Vec<Reward>, reqwest::Error> {
    let rewards = reqwest::get("https://api.hederacourt.site/api/v1/rewards")
        .await?
        .json::<Vec<Reward>>()
        .await?;

    Ok(rewards)
}

// Retrieves regions from the ENDA backend API
pub async fn get_regions() -> Result<Vec<Region>, reqwest::Error> {
    let regions = reqwest::get("https://api.hederacourt.site/api/v1/regions")
        .await?
        .json::<Vec<Region>>()
        .await?;

    Ok(regions)
}
