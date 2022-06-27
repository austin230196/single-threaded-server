use std::{io::prelude::*, net::{TcpListener, TcpStream}, fs, path::Path};



fn main(){
    let uri = "127.0.0.1:9000";
    let listener = TcpListener::bind(&uri).unwrap();
    println!("Server started on http://{}", &uri);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}



fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("{}", &request);
    let method = helpers::get_method(&request).unwrap();
    let url = helpers::get_url(&request).unwrap();
    println!("Method: {:#?}, Url: {:#?}", &method, &url);
    let response = helpers::response(200, "index.html".to_owned(), "OK".to_owned());
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}



mod helpers {
    use super::*;

    pub fn get_method<'a>(request: &'a str) -> Option<&'a str> {
        let mut iter = request.split_whitespace();
        let method = iter.next();
        method
    }
    
    
    pub fn get_url<'a>(request: &'a str) -> Option<&'a str> {
        let mut iter = request.split_whitespace();
        iter.next();
        let url = iter.next();
        url
    }

    pub fn response(code: u32, file: String, status: String) -> String {
        let mut response_template = "HTTP/1.1 %code% %status% \r\nContent-Length:%len%\r\n\r\n%body%".to_owned();
        let p = Path::new("./").join(file);
        let content = fs::read_to_string(p).unwrap();
        response_template = response_template.replace("%code%", format!("{}", code).as_str());
        response_template = response_template.replace("%status%", format!("{}", status).as_str());
        response_template = response_template.replace("%len%", format!("{}", content.len()).as_str());
        response_template = response_template.replace("%body%", content.as_str());
        response_template
    }
}