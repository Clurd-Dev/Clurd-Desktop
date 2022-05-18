use std::fs;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
pub fn parser() -> String {
    let contents = fs::read_to_string("./config.json");
    let result = match contents {
      Ok(contents) => contents,
      Err(_error) => String::from("0"),
    };
    if result != "0"{
      let json: Result<serde_json::Value, serde_json::Error> =serde_json::from_str(&result);
      let is_parsed = match json {
        Ok(json) => format!("{}" , json),
        Err(_error) => String::from("0"),
      };
      is_parsed
    }else{
      String::from("0")
    }
}

pub fn update(invoke_message: String ){
  let config = json!({
    "url": invoke_message
  });
  let mut file = File::create("./config.json").expect("Error during creating file");
  file.write_all(format!("{}", config).as_bytes()).expect("Error during writing");
}

pub async fn get_config_from_server(url: String) -> String{
  let response = reqwest::get(url).await;
  let response_result = match response {
    Ok(response) => response.text().await.expect("Error during fetch config"),
    Err(_error) => String::from("0")
  };
  response_result
}