mod conf;
mod sensor_reading;
mod readout;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Method};
use tokio::time::{sleep, Duration};
use log::{error, info};

#[tokio::main]
async fn main() {
  let delay: u64 = 10;
  let api_config = conf::get_api_config();
  let mut headers = HeaderMap::new();
  let auth_value = format!("Bearer {}", &api_config[1]);
  headers.insert(
    AUTHORIZATION, 
    HeaderValue::from_str(&auth_value).unwrap()
  );

  let client = Client::new();
  let request_template = client
    .request(Method::POST, &api_config[0])
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