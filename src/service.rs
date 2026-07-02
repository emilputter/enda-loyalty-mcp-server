use crate::models::{ClientClasses, Region, Reward};

fn api_main_url() -> String {
    std::env::var("API_MAIN_URL")
        .expect("API_MAIN_URL not found")
}

// Retrieves client classes from the ENDA backend API
pub async fn get_client_classes() -> Result<Vec<ClientClasses>, reqwest::Error> {
    let classes = reqwest::get(format!("{}/client-classes", api_main_url()))
        .await?
        .json::<Vec<ClientClasses>>()
        .await?;

    Ok(classes)
}

// Retrieves rewards from the ENDA backend API
pub async fn get_rewards() -> Result<Vec<Reward>, reqwest::Error> {
    let rewards = reqwest::get(format!("{}/rewards", api_main_url()))
        .await?
        .json::<Vec<Reward>>()
        .await?;

    Ok(rewards)
}

// Retrieves regions from the ENDA backend API
pub async fn get_regions() -> Result<Vec<Region>, reqwest::Error> {
    let regions = reqwest::get(format!("{}/regions", api_main_url()))
        .await?
        .json::<Vec<Region>>()
        .await?;

    Ok(regions)
}
