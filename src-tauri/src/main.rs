#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod server_io;
mod config_manipolation;
mod sync;
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
async fn ftp_get(url: String) -> String {
    sync::get()
}

#[tauri::command]
async fn ftp_put(url: String, files_to_upload: String, filename: String) -> String {
    String::from(sync::put(url, files_to_upload, filename))
}

#[tauri::command]
async fn copy_fs(url: String, old: String, new: String) ->  String {
  let result = server_io::copy_file(url, old , new).await;
  result
}

#[tauri::command]
async fn move_fs(url: String, old: String, new: String) ->  String {
  let result = server_io::move_file(url, old , new).await;
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

#[tauri::command]
async fn get_space(url: String, path: String) ->  String {
  let response = server_io::get_space(url, path).await;
  response
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![getfiles,update_config,
      parse_config, get_config, remove_fs, 
      rename_fs, get_space, copy_fs, move_fs, ftp_get,ftp_put])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
