extern crate hyper;
extern crate libsnatch;

use libsnatch::contentlength::GetContentLength;
use libsnatch::client::GetResponse;
use hyper::client::Client;
use libsnatch::webfile::WebFile;

fn main() {
    let url = "http://www.cbu.edu.zm/downloads/pdf-sample.pdf";
    let authorized_threads: usize = 8;
    let server_response = Client::send_request_for_file(url);
    let headers = match server_response {
        Ok(body) => body.headers.clone(),
        Err(error) => panic!("Error: {:?}", error),
    };
    let content_length = match headers.get_content_length() {
        Some(length) => length,
        None => panic!("Error: canno't get the length from the webpage {}", url),
    };
    println!("The content length of the file contained in {} is {} bytes",
             url,
             content_length);
    let webfile = WebFile::new(authorized_threads,
                               content_length,
                               "SimplePDF.pdf",
                               "./test/",
                               url);
    let chunks = webfile.download_chunks();

    match webfile.save_chunks(&chunks) {
        Ok(_) => println!("Download OK!"),
        Err(e) => println!("Download failed : {:?}", e),
    }
}