use vertigo::{DomDriver};

pub async fn fetch_root(driver: DomDriver) -> Result<String, String> {
    let url = format!("/fetch_root");

    let response = driver.fetch(url).get().await;

    response
}
