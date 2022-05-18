#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod server_io;
mod config_manipolation;

#[tauri::command]
async fn getfiles(invoke_message: String, path: String) ->  String {
  let files = server_io::getfiles(path, invoke_message).await;
  files
}


#[tauri::command]
async fn remove_fs(url: String, absolute: String) ->  String {
  let result = server_io::remove_file(absolute, url).await;
  result
}

#[tauri::command]
async fn rename_fs(url: String, old: String, new: String) ->  String {
  let result = server_io::rename_file(old, url, new).await;
  result
}

#[tauri::command]
async fn parse_config() ->  String {
  let config = config_manipolation::parser();
  config
}

#[tauri::command]
async fn get_config(url: String) ->  String {
  let response = config_manipolation::get_config_from_server(url).await;
  response
}


#[tauri::command]
async fn update_config(invoke_message: String) ->  () {
  let result = config_manipolation::update(invoke_message);
  result
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![getfiles,update_config,parse_config, get_config, remove_fs, rename_fs])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
