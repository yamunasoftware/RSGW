mod sensor_reading;
mod readout;

use std::env;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Method};
use tokio::time::{sleep, Duration};
use log::{error, info};

#[tokio::main]
async fn main() {
  if let Err(e) = dotenvy::from_filename("config.env") {
    error!("Failed to Load Configuration File");
  }

  let delay: u64 = 10;
  let api_url = env::var("API_URL").unwrap();
  let api_key = env::var("API_KEY").unwrap();

  let mut headers = HeaderMap::new();
  let auth_value = format!("Bearer {}", api_key.clone());
  headers.insert(
    AUTHORIZATION, 
    HeaderValue::from_str(&auth_value).unwrap()
  );

  let client = Client::new();
  let request_template = client
    .request(Method::POST, api_url.clone())
    .headers(headers);

  loop {
    let sensor_readings = readout::data_readout();
    let response = request_template
      .try_clone()
      .expect("Failed to Clone Request Template")
      .json(&sensor_readings)
      .send()
      .await
      .unwrap();

    let status = response.status();
    if status.is_success() { info!("Publish Successful."); }
    else { error!("Publish Failed, Status {}", status); }
    sleep(Duration::from_secs(delay)).await;
  }
}