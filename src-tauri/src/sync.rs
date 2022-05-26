extern crate suppaftp;

use suppaftp::FtpStream;
use std::convert::TryFrom;
use suppaftp::{ list::File};
use std::io::Read;

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
