extern crate suppaftp;

use suppaftp::FtpStream;
use std::convert::TryFrom;
use suppaftp::{ list::File};
use std::io::Read;
use std::io;
extern crate reqwest;


pub fn put(url: String, file_for_server: String, filename: String) -> String {
    let filedef = file_for_server.to_string();
    let mut ftp_stream = FtpStream::connect("localhost:21").unwrap();
    ftp_stream.login("admin", "admin").unwrap();
    let mut f = std::fs::File::open(&filedef).expect("no file found");
    let metadata = std::fs::metadata(&filedef).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    let mut temp = buffer.clone();
    let mut u8_bytes = unsafe { &*(&mut temp[..] as *mut[u8] as *mut[u8]) };
    let d = ftp_stream.put_file(filename, &mut u8_bytes);
    println!("{:?}", d);
    let result = match d{
        Ok(_d) => "1",
        Err(_err) => "0",
    };
    String::from(result)
}

pub async fn download(url: String, filename: String, savefolder: String){
    let resp = reqwest::get(url).await.expect("request failed");
    let body = resp.text().await.expect("body invalid");
    let mut out = std::fs::File::create(format!("{}/{}", savefolder, filename)).expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
}