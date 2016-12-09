extern crate argparse;
extern crate hyper;
extern crate libsnatch;

use argparse::{ArgumentParser, Print, Store, StoreTrue};
use hyper::client::Client;
use libsnatch::client::GetResponse;
use libsnatch::contentlength::GetContentLength;
use libsnatch::webfile::WebFile;

fn main() {

    let mut verbose = false;
    let mut output_directory = String::from("./test/");
    let mut output_file = String::from("test_file");
    let mut retry_times: usize = 3;
    let mut url = String::from("");

    {
        let mut app = ArgumentParser::new();
        app.set_description("A simple, fast and interruptable download accelerator.");
        app.add_option(&["-V", "--version"],
                       Print(env!("CARGO_PKG_VERSION").to_string()),
                       "Show version");
        app.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        app.refer(&mut output_directory).add_option(&["-O", "--output_directory"],
                                                    Store,
                                                    "Path to the output directory");
        app.refer(&mut output_file).add_option(&["-o", "--output_file"], Store, "Output file name");
        // app.refer(&mut retry_times).add_option(&["r", "--retry"],
        //                                       Store,
        //                                       "Set the number of times to retry a failed \
        //                                        download");
        app.refer(&mut url)
            .add_option(&["-u", "--url"], Store, "The URL to download the file")
            .required();
        app.parse_args_or_exit();
    }

    let authorized_threads: usize = 8;
    let server_response = Client::send_request_for_file(&url);
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
                               &output_file,
                               &output_directory,
                               retry_times,
                               &url);
    let chunks = webfile.download_chunks();

    match webfile.save_chunks(&chunks) {
        Ok(_) => println!("Download OK!"),
        Err(e) => println!("Download failed : {:?}", e),
    }
}