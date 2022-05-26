extern crate suppaftp;

use suppaftp::FtpStream;
use std::convert::TryFrom;
use suppaftp::{ list::File};

pub fn get() -> String{
    let mut ftp_stream = FtpStream::connect("localhost:21").unwrap();
    let mut file_name: Vec<String> = Vec::new();
    ftp_stream.login("admin", "admin").unwrap();
    let ls2: Vec<File> = ftp_stream.list(None).ok().unwrap().iter().map(|x| File::try_from(x.as_str()).ok().unwrap()).collect();
    assert!(ftp_stream.quit().is_ok());
    let ls3 = &ls2;
    for file in ls3{
        let name = format!("{:?}", file.name());
        file_name.push(name);
    }
    format!("{:?}", file_name)
}

pub fn put(url: String, file_for_server: String, bytes: & [u8]) -> &str {
    let mut ftp_stream = FtpStream::connect("localhost:21").unwrap();
    ftp_stream.login("admin", "admin").unwrap();
    let mut bytes_file = bytes.clone();
    let d = ftp_stream.put_file(file_for_server, &mut bytes_file);
    let result = match d{
        Ok(_d) => "1",
        Err(_err) => "0",
    };
    result
}
