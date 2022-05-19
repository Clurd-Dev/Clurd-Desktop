use serde_json::json;
pub async fn getfiles(path: String, invoke_message: String) -> String{
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

pub async fn remove_file(absolute: String, url: String) -> String{
    let client = reqwest::Client::new();
    let body = json!({
      "folder": absolute
    });
    let response = client.post(url).body(format!("{}", body)).send().await;
    let response_result = match response {
      Ok(response) => response.text().await.expect("CIAO"),
      Err(_error) => String::from("0")
    };
    println!("{}", response_result);
    response_result
}

pub async fn rename_file(old: String, url: String, new_file: String) -> String{
  let client = reqwest::Client::new();
  let body = json!({
    "folder": old,
    "new": new_file
  });
  let response = client.post(url).body(format!("{}", body)).send().await;
  let response_result = match response {
    Ok(response) => response.text().await.expect("CIAO"),
    Err(_error) => String::from("0")
  };
  println!("{}", response_result);
  response_result
}

pub async fn get_space(url: String, path: String) -> String{
  let client = reqwest::Client::new();
  let body = json!({
    "folder": path
  });
  let response = client.post(url).body(format!("{}", body)).send().await;
  let response_result = match response {
    Ok(response) => response.text().await.expect("CIAO"),
    Err(_error) => String::from("0")
  };
  println!("{}", response_result);
  response_result
}