#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::fs;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
#[tauri::command]
async fn getfiles(invoke_message: String, path: String) ->  String {
  //println!("{}", invoke_message);
  let client = reqwest::Client::new();
  let body = json!({
    "folder": path
  });
  let response = client.post(invoke_message).body(format!("{}", body)).send().await;
  let response_result = match response {
    Ok(response) => response.text().await.expect("CIAO"),
    Err(_error) => String::from("0")
  };
  response_result
}
#[tauri::command]
async fn get_config(url: String) ->  String {
  let response = reqwest::get(url).await;
  let response_result = match response {
    Ok(response) => response.text().await.expect("Error during fetch config"),
    Err(_error) => String::from("0")
  };
  response_result
}
#[tauri::command]
async fn remove_fs(url: String, absolute: String) ->  String {
  let client = reqwest::Client::new();
  let body = json!({
    "folder": absolute
  });
  let response = client.post(url).body(format!("{}", body)).send().await;
  let response_result = match response {
    Ok(response) => response.text().await.expect("CIAO"),
    Err(_error) => String::from("0")
  };
  response_result
}
#[tauri::command]
async fn parse_config() ->  String {
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

#[tauri::command]
async fn update_config(invoke_message: String) ->  () {
  let config = json!({
    "url": invoke_message
  });
  let mut file = File::create("./config.json").expect("Error during creating file");
  file.write_all(format!("{}", config).as_bytes()).expect("Error during writing");
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![getfiles,update_config,parse_config, get_config, remove_fs])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
