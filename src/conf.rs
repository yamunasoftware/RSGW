use std::fs;

pub fn get_api_config() -> [String; 2] {
  let mut config = [const { String::new() }; 2];
  let content = fs::read_to_string("/main/resources/.conf").unwrap();

  for line in content.lines() {
    if line.contains("API_URL=") {
      config[0] = line.replace("API_URL=", "").trim().to_string();
    }

    else if line.contains("API_KEY=") {
      config[1] = line.replace("API_KEY=", "").trim().to_string();
    }
  }
  config
}

pub fn get_system_config() -> [String; 2] {
  let mut config = [const { String::new() }; 2];
  let content = fs::read_to_string("/main/resources/.conf").unwrap();

  for line in content.lines() {
    if line.contains("SYSTEM_ID=") {
      config[0] = line.replace("SYSTEM_ID=", "").trim().to_string();
    }

    else if line.contains("SYSTEM_TYPE=") {
      config[1] = line.replace("SYSTEM_TYPE=", "").trim().to_string();
    }
  }
  config
}